use std::ops::Deref;
use std::borrow::{ToOwned, Borrow};
use std::str::from_utf8;
use std::hash::{Hash, Hasher};
use std::convert::AsRef;

const DOUBLE_WORD_SIZE: usize = 2 * std::mem::size_of::<isize>();

/// Returned when trying to convert a &str into a InlineStr
/// but it fails because it doesn't fit.
#[derive(Debug)]
pub struct StringTooLongError;

#[derive(Debug, Clone, Copy, Eq)]
pub struct InlineStr {
    inner: [u8; DOUBLE_WORD_SIZE],
}

impl<'a> AsRef<str> for InlineStr {
    fn as_ref(&self) -> &str {
        self.deref()
    }
}

impl Hash for InlineStr {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.deref().hash(state);
    }
}

impl From<char> for InlineStr {
    fn from(c: char) -> Self {
        let mut inner = [0u8; DOUBLE_WORD_SIZE];
        c.encode_utf8(&mut inner);
        inner[DOUBLE_WORD_SIZE - 1] = c.len_utf8() as u8;
        Self { inner }
    }
}

impl<'a> std::cmp::PartialEq<InlineStr> for InlineStr {
    fn eq(&self, other: &InlineStr) -> bool {
        self.deref() == other.deref()
    }
}

// This could be an implementation of TryFrom<&str>
// when that trait is stabilized.
impl InlineStr {
    pub fn try_from_str(s: &str) -> Result<InlineStr, StringTooLongError> {
        let len = s.len();
        if len < DOUBLE_WORD_SIZE {
            let mut inner = [0u8; DOUBLE_WORD_SIZE];
            inner[..len].copy_from_slice(s.as_bytes());
            inner[DOUBLE_WORD_SIZE - 1] = len as u8;
            Ok(Self { inner })
        } else {
            Err(StringTooLongError)
        }
    }
}

impl Deref for InlineStr {
    type Target = str;

    fn deref(&self) -> &str {
        let len = self.inner[DOUBLE_WORD_SIZE - 1] as usize;
        from_utf8(&self.inner[..len]).unwrap()
    }
}

#[derive(Debug, Eq)]
pub enum CowStr<'a> {
    Boxed(Box<str>),
    Borrowed(&'a str),
    Inlined(InlineStr),
}

impl<'a> AsRef<str> for CowStr<'a> {
    fn as_ref(&self) -> &str {
        self.deref()
    }
}

impl<'a> Hash for CowStr<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.deref().hash(state);
    }
}

impl<'a> std::clone::Clone for CowStr<'a> {
    fn clone(&self) -> Self {
        match self {
            CowStr::Boxed(s) if s.len() < DOUBLE_WORD_SIZE
                => CowStr::Inlined(InlineStr::try_from_str(&**s).unwrap()),
            CowStr::Boxed(s) => CowStr::Boxed(s.clone()),
            CowStr::Borrowed(s) => CowStr::Borrowed(s),
            CowStr::Inlined(s) => CowStr::Inlined(*s),
        }
    }
}

impl<'a> std::cmp::PartialEq<CowStr<'a>> for CowStr<'a> {
    fn eq(&self, other: &CowStr) -> bool {
        self.deref() == other.deref()
    }
}

impl<'a> From<&'a str> for CowStr<'a> {
    fn from(s: &'a str) -> Self {
        CowStr::Borrowed(s)
    }
}

impl<'a> From<String> for CowStr<'a> {
    fn from(s: String) -> Self {
        CowStr::Boxed(s.into_boxed_str())
    }
}

impl<'a> From<char> for CowStr<'a> {
    fn from(c: char) -> Self {
        CowStr::Inlined(c.into())
    }
}

impl<'a> Deref for CowStr<'a> {
    type Target = str;

    fn deref(&self) -> &str {
        match self {
            CowStr::Boxed(ref b) => &*b,
            CowStr::Borrowed(b) => b,
            CowStr::Inlined(ref s) => s.deref(),
        }
    }
}

impl<'a> Borrow<str> for CowStr<'a> {
    fn borrow(&self) -> &str {
        self.deref()
    }
}

impl<'a> CowStr<'a> {
    pub fn to_string(self) -> String {
        match self {
            CowStr::Boxed(b) => b.into(),
            CowStr::Borrowed(b) => b.to_owned(),
            CowStr::Inlined(s) => s.deref().to_owned(),
        }        
    }
}

#[cfg(test)]
mod test_special_string {
    use super::*;

    #[test]
    fn inlinestr_ascii() {
        let s: InlineStr = 'a'.into();
        assert_eq!("a", s.deref());
    }

    #[test]
    fn inlinestr_unicode() {
        let s: InlineStr = '🍔'.into();
        assert_eq!("🍔", s.deref());
    }

    #[test]
    fn cowstr_size() {
        let size = std::mem::size_of::<CowStr>();
        let word_size = std::mem::size_of::<isize>();
        assert_eq!(3 * word_size, size);
    }

    #[test]
    fn cowstr_char_to_string() {
        let c = '藏';
        let smort: CowStr = c.into();
        let owned: String = smort.to_string();
        let expected = "藏".to_owned();
        assert_eq!(expected, owned);
    }

    #[test]
    fn double_word_size_atleast_five() {
        // we need 4 bytes to store a char and then one more to store
        // its length
        assert!(DOUBLE_WORD_SIZE >= 5);
    }

    #[test]
    #[cfg(target_pointer_width = "64")]
    fn inlinestr_fits_fifteen() {
        let s = "0123456789abcde";
        let stack_str = InlineStr::try_from_str(s).unwrap();
        assert_eq!(stack_str.deref(), s);
    }

    #[test]
    #[cfg(target_pointer_width = "64")]
    fn inlinestr_not_fits_sixteen() {
        let s = "0123456789abcdef";
        let _stack_str = InlineStr::try_from_str(s).unwrap_err();
    }

    #[test]
    #[cfg(target_pointer_width = "64")]
    fn small_boxed_str_clones_to_stack() {
        let s = "0123456789abcde".to_owned();
        let smort: CowStr = s.into();
        let smort_clone = smort.clone();

        if let CowStr::Inlined(..) = smort_clone {} else {
            panic!("Expected a Inlined variant!");
        }
    }
}

