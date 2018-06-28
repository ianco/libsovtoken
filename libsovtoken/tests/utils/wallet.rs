/*!
 * A set of test helpers dealing with the wallet.
 */

extern crate rust_indy_sdk as indy;
extern crate sovtoken;

use self::indy::ErrorCode;
use self::indy::wallet::Wallet as IndyWallet;
use self::sovtoken::utils::random::rand_string;

static USEFUL_CREDENTIALS : &'static str = r#"
   {
       "key": "12345678901234567890123456789012",
       "rekey": null,
       "storage": null
   }
"#;


pub struct Wallet {
    name: String,
    pub handle: i32,
}

impl Wallet {
    pub fn new() -> Wallet {
        let name = rand_string(20);
        let mut wallet = Wallet { name, handle: -1 };
        wallet.create().unwrap();
        wallet.open().unwrap();

        wallet
    }

    pub fn from_name(name: &str) -> Wallet {
        let name = name.to_owned();
        let mut wallet = Wallet { name, handle: -1 };
        wallet.create().unwrap();
        wallet.open().unwrap();

        wallet
    }

    fn open(&mut self) -> Result<i32, ErrorCode> {
        let handle = IndyWallet::open(&self.name, None, Some(USEFUL_CREDENTIALS))?;
        self.handle = handle;
        Ok(handle)
    }

    fn create(&self) -> Result<(), ErrorCode> {
        IndyWallet::create("pool_1", &self.name, None, None, Some(USEFUL_CREDENTIALS))
    }

    fn close(&self) -> Result<(), ErrorCode> {
        IndyWallet::close(self.handle)
    }

    fn delete(&self) -> Result<(), ErrorCode> {
        IndyWallet::delete(&self.name, Some(USEFUL_CREDENTIALS))
    }
}

impl Drop for Wallet {
    fn drop(&mut self) {
        self.close().unwrap();
        self.delete().unwrap();
    }
}
