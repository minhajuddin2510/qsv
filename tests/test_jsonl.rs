use crate::workdir::Workdir;

#[test]
fn jsonl_simple() {
    let wrk = Workdir::new("jsonl");
    wrk.create_from_string(
        "data.jsonl",
        r#"{"id":1,"father":"Mark","mother":"Charlotte","oldest_child":"Tom"}
{"id":2,"father":"John","mother":"Ann","oldest_child":"Jessika"}
{"id":3,"father":"Bob","mother":"Monika","oldest_child":"Jerry"}"#,
    );
    let mut cmd = wrk.command("jsonl");
    cmd.arg("data.jsonl");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["id", "father", "mother", "oldest_child"],
        svec!["1", "Mark", "Charlotte", "Tom"],
        svec!["2", "John", "Ann", "Jessika"],
        svec!["3", "Bob", "Monika", "Jerry"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn jsonl_nested() {
    let wrk = Workdir::new("jsonl");
    wrk.create_from_string(
        "data.jsonl",
        r#"{"id":1,"father":"Mark","mother":"Charlotte","children":["Tom"]}
{"id":2,"father":"John","mother":"Ann","children":["Jessika","Antony","Jack"]}
{"id":3,"father":"Bob","mother":"Monika","children":["Jerry","Karol"]}"#,
    );
    let mut cmd = wrk.command("jsonl");
    cmd.arg("data.jsonl");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["id", "father", "mother", "children"],
        svec!["1", "Mark", "Charlotte", "\"Tom\""],
        svec!["2", "John", "Ann", "\"Jessika\",\"Antony\",\"Jack\""],
        svec!["3", "Bob", "Monika", "\"Jerry\",\"Karol\""],
    ];
    assert_eq!(got, expected);
}
