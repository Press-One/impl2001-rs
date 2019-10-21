#[macro_use(lazy_static)]
extern crate lazy_static;

pub mod pip;
//pub use crate::pip::pip2001::Pip2001;

#[cfg(test)]
mod tests {
    use crate::pip::Pip;
    use crate::pip::PipId;
    use super::pip::pip2001::Pip2001;

    #[test]
    fn test_runpip() {
        let mut p:Pip2001 = Pip2001::new();
        assert_eq!(PipId::PIP2001, p.Type);
    }

    #[test]
    fn test_json_to_pip2001() {
        let mut p:Pip2001 = Pip2001::new();
        let jsonstr = r#"
        {
            "topic":"6d318BE3657FBc0515a0ab4945c21Be0f17D935f",
            "allow":"1800246B58dC9EdF07013b66A5A6800a9596F419,3dDDE8416F24BcCAc86505Eb598fd455C7C2bf74,697667E0c877c6F9927Ebb171058ADdA76A82424"
        }
        "#;
        let p2 = p.from_json(&jsonstr);
        match p2{
            Ok(Some(v)) => {
                dbg!(v.data.get("topic"));
                assert_eq!(String::from("6d318BE3657FBc0515a0ab4945c21Be0f17D935f"), v.data["topic"]);
                let p2json = v.to_json();
                assert!(p2json.contains("6d318BE3657FBc0515a0ab4945c21Be0f17D935f"));
            },
            Ok(None) => {
                panic!("Pip2001.from_json return None");
            },
            Err(e)=> panic!("{:?}",e),
        }
    }
}

