use cli::scaffold::Scaffold;
use error::DefaultResult;
use std::path::Path;
use util;

pub struct RustScaffold;

impl Scaffold for RustScaffold {
    fn gen<P: AsRef<Path>>(&self, base_path: P) -> DefaultResult<()> {
        util::run_cmd(
            base_path.as_ref().to_path_buf(),
            "cargo".into(),
            vec!["init".to_owned()],
        )?;

        Ok(())
    }
}
