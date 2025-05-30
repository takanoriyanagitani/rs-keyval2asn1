use std::io;

use io::Read;

use io::Write;

use der::asn1::OctetStringRef;

#[derive(der::Sequence)]
pub struct KeyValPair<'a> {
    pub key: OctetStringRef<'a>,
    pub val: OctetStringRef<'a>,
}

impl<'a> KeyValPair<'a> {
    pub fn to_writer<W>(&self, wtr: &mut W) -> Result<(), io::Error>
    where
        W: Write,
    {
        der::Encode::encode(self, wtr).map_err(io::Error::other)
    }
}

pub fn kv2der2writer<W>(key: &[u8], val: &[u8], wtr: &mut W) -> Result<(), io::Error>
where
    W: Write,
{
    let kvp = KeyValPair {
        key: OctetStringRef::new(key).map_err(io::Error::other)?,
        val: OctetStringRef::new(val).map_err(io::Error::other)?,
    };
    kvp.to_writer(wtr)
}

pub fn stdin2val_limited_new(limit: u64) -> impl FnMut() -> Result<Vec<u8>, io::Error> {
    move || {
        let i = io::stdin();
        let l = i.lock();
        let mut limited = l.take(limit);
        let mut buf: Vec<u8> = vec![];
        limited.read_to_end(&mut buf)?;
        Ok(buf)
    }
}

pub fn env2key(name_of_the_key: &'static str) -> impl FnMut() -> Result<Vec<u8>, io::Error> {
    move || {
        let rval: Result<_, io::Error> =
            std::env::var_os(name_of_the_key).ok_or(io::Error::other("no key got"));
        let val = rval?;
        Ok(val.into_encoded_bytes())
    }
}

pub fn stdin2key2env2val2der2stdout(
    name_of_the_key: &'static str,
    val_limit: u64,
) -> Result<(), io::Error> {
    let key: Vec<u8> = env2key(name_of_the_key)()?;
    let val: Vec<u8> = stdin2val_limited_new(val_limit)()?;
    let o = io::stdout();
    let mut l = o.lock();
    kv2der2writer(&key, &val, &mut l)?;
    l.flush()
}

pub const NAME_OF_THE_KEY_DEFAULT: &str = "ENV_KEY";
pub const VAL_SIZE_LIMIT_DEFAULT: u64 = 1048576;

pub fn stdin2key2env2val2der2stdout_default() -> Result<(), io::Error> {
    stdin2key2env2val2der2stdout(NAME_OF_THE_KEY_DEFAULT, VAL_SIZE_LIMIT_DEFAULT)
}
