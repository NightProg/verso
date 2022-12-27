use std::collections::HashMap;

pub trait QueryWeb: std::fmt::Display {

    fn from_query_map(&self) -> HashMap<String, String> {
        let string_self = self.to_string();
        let split_element = string_self.split("&").by_ref().collect::<Vec<_>>();
        let mut obj = HashMap::new();

        if split_element.len() == 0 {
            let split_e = string_self.split("=").by_ref().collect::<Vec<_>>();
            if split_e.len() == 2 {
                obj.insert(split_e[0].to_string(), split_e[1].to_string());
            }
            return obj;

        } else {
            for i in split_element.clone() {
                let e = i.split("=").by_ref().collect::<Vec<_>>();
                if e.len() == 2 {
                    obj.insert(e[0].to_string(),e[1].to_string());
                }
            }
        }

        obj
    }

    fn from_json(&self) -> Result<HashMap<String,String>, String>{
        let split_self = self.to_string();
        if ! (split_self.starts_with("{") && split_self.ends_with("}")) {
            return Err("unclosed { or }".to_string());
        }
        let mut object = HashMap::new();
        let dict_body = &split_self[1..split_self.len()-1];
        let split_dict_body = dict_body.split(",");
        for i in split_dict_body {
            let element = i.split(":").by_ref().collect::<Vec<_>>();
            if element.len() <= 1 {
                return Err("Json parse error".to_string());
            }
            object.insert(element[0].to_string(),element[1].to_string());

        }

        return Ok(object);

    }
}
impl<E: std::fmt::Display> QueryWeb for E {}