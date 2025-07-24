fn copy_and_return<'a>(vector: &'a mut Vec<String>, value: &'a str) -> &'a String{
    vector.push(String::from(value));
    vector.get(vector.len() - 1).unwrap()
}

fn main() {
    let name1 = "karla";
    let name2 = "david";
    let name3 = "ana";

    let mut names = Vec::new();

    assert_eq!("karla", copy_and_return(&mut names, &name1));
    assert_eq!("david", copy_and_return(&mut names, &name2));
    assert_eq!("ana", copy_and_return(&mut names, &name3));

    let m = assert_eq!(names, vec!["karla".to_string(), "david".to_string(), "ana".to_string()]);
    println!("copy an return: {:?}", m);

}
