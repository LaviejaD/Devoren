// parser  the url  and
// http://127.0.0.1:8080/ => [/]
// http://127.0.0.1:8080/user/<id> =>[/user, /<id>]
pub fn url_split(param: String) -> Vec<String> {
    let mut result = Vec::new();
    let mut temp = Vec::new();
    for c in param.clone().chars() {
        if c.is_whitespace() {
            continue;
        }
        // if c == / and temp  > 0  add new string to result
        if c == '/' && temp.len() > 0 {
            result.push(temp.clone().iter().collect());
            temp.clear();
            temp.push(c);

            continue;
        }

        temp.push(c);
    }
    if temp.len() > 0 {
        result.push(temp.iter().collect());
    }
    result
}
//check if is  dinamy parameter
pub fn has_dinamy_params(param: String) -> bool {
    let mut start = false;
    let mut end = false;
    for pchar in param.chars() {
        if '<' == pchar {
            start = true;
        }
        if '>' == pchar {
            end = true;
        }
    }
    start == true && end == true
}
//get all dinamy param position
pub fn dinamy_params_len(dinamy: Vec<String>) -> Vec<bool> {
    let mut result = Vec::new();
    for p in 0..dinamy.len() {
        result.push(dinamy[p].starts_with("/<") && dinamy[p].ends_with(">"));
    }
    result
}
// /user/<id>
// /user/1
pub fn equal_url(dinamy: Vec<String>, request_url: Vec<String>) -> bool {
    let u1_p = dinamy_params_len(dinamy.clone());
    let mut equal = 0;
    // if has equal numbers of parameter continuel return false
    if dinamy.len() != request_url.len() {
        return false;
    }
    for i in 0..dinamy.len() {
        if dinamy[i] == request_url[i] {
            equal += 1;
        }
        // if "p1_p" is true and is the current position is "i" then  "equal+=1"
        if u1_p[i] {
            equal += 1
        }
    }
    equal == dinamy.len()
}
// /user/<id>
// /user/123
pub fn get_url_params_and_value(
    params_name: Vec<String>,
    params_value: Vec<String>,
) -> Vec<(String, String)> {
    let mut result = Vec::new();
    for i in 0..params_name.len() {
        result.push((
            params_name[i].clone().replace("/<", "").replace(">", ""),
            params_value[i].clone().replace("/", ""),
        ));
    }

    result
}
