#[cfg(test)]
mod tests {

    use include_one::{config::Config, error::SwayIOError, parse_configs};
    use std::{collections::HashSet, path::PathBuf};

    //We shell expand before because OS args will always be shell expanded in main args

    #[test]
    fn wrong_path() {
        let mut c = Config::default();
        let p = ["NO_WAY_THIS_EXISTS".to_string()];
        let r = parse_configs(&mut p.into_iter(), &mut c);
        assert_eq!(
            r,
            Err(SwayIOError::PathNotFound {
                path: "NO_WAY_THIS_EXISTS".to_string()
            })
        )
    }

    #[test]
    fn single_entry_config() {
        let mut c = Config::default();
        let st = shellexpand::full("$PWD/test_confs/include_conf.d/first.conf")
            .unwrap()
            .to_string();
        let mut c_set = HashSet::new();
        c_set.insert(PathBuf::from(st.clone()));
        let p = [st];
        let r = parse_configs(&mut p.into_iter(), &mut c);
        assert_eq!(r, Ok(()));
        assert_eq!(
            c,
            Config::new(true, "\"Times New Roman\"".to_string(), c_set)
        );
    }

    #[test]
    fn full_dir() {
        let mut c = Config::default();
        let st = shellexpand::full("$PWD/test_confs/include_conf.d/test.conf").unwrap().to_string();
        let mut c_set = HashSet::new();
        c_set.insert(PathBuf::from(st.clone()));
        c_set.insert(PathBuf::from(shellexpand::full("$PWD/test_confs/include_conf.d/first.conf").unwrap().to_string()));
        let p = [st];
        let r = parse_configs(&mut p.into_iter(), &mut c);
        assert_eq!(r, Ok(()));
        assert_eq!(
            c,
            Config::new(true, "\"Times New Roman\"".to_string(), c_set)
        );
    }

    #[test]
    fn wrong_format() {
        let mut c = Config::default();
        let st = shellexpand::full("$PWD/test_confs/wrong_conf.d/bad_format.conf").unwrap().to_string();
        let mut c_set = HashSet::new();
        c_set.insert(PathBuf::from(st.clone()));
        let p = [st];
        let r = parse_configs(&mut p.into_iter(), &mut c);
        assert_eq!(r, Err(SwayIOError::UnknownDirective { line: "asdsd".to_string() }));
    }

    #[test]
    fn include_one_test() {
        let mut c = Config::default();
        let st = shellexpand::full("$PWD/test_confs/include_one_conf.d/runme.conf").unwrap().to_string();
        let mut c_set = HashSet::new();
        c_set.insert(PathBuf::from(st.clone()));
        c_set.insert(PathBuf::from(shellexpand::full("$PWD/test_confs/include_one_conf.d/a/first.conf").unwrap().to_string()));
        let p = [st];
        let r = parse_configs(&mut p.into_iter(), &mut c);
        assert_eq!(r, Ok(()));
        assert_eq!(
            c,
            Config::new(true, "\"Helvetica\"".to_string(), c_set)
        );
    }
    
}
