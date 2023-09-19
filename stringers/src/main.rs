
fn appends_question_owned(s: String) -> String
{
    let s2 = s + ", you don't say?";
    s2
}


fn appends_question_borrowed(s: &str) -> String
{
    let mut s2 = s.to_string();
    s2 += ", you don't say?";
    s2
}

fn appends_question_mut_borrowed(s: &mut String)
{
 s.push_str(", you don't say?");
}

fn appends_question_mut_borrowed_with_return(s: &mut String) -> String
{
    s.push_str(", you don't say?");
    s.clone()
}

fn string_length_borrowed(s: &String)-> usize
{
    s.len()
}
fn string_length_owned(s: String)-> usize
{
    s.len()
}

fn main() {
    let s = String::from("You're late");

    let s = appends_question_owned(s);
    println!("{}", s);

    let mut s_borrowed = appends_question_borrowed(&s);
    println!("{}", s_borrowed);

    appends_question_mut_borrowed(&mut s_borrowed);
    println!("{}", s_borrowed);

    let s_mut_borrowed_return = appends_question_mut_borrowed_with_return(&mut s_borrowed);
    println!("{}", s_mut_borrowed_return);

    println!("s length: {}", string_length_borrowed(&s));
    println!("s length: {}", string_length_owned(s));
}
