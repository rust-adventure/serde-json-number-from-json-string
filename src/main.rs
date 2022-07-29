use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[serde_as]
#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    #[serde_as(as = "Option<DisplayFromStr>")]
    phones: Option<i64>,
}

fn main() -> Result<(), serde_json::Error> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        {
            "name": "John Doe",
            "phones": "780495989403287554"
            
        }"#;

    // Parse the string of data into a Person object. This is exactly the
    // same function as the one that produced serde_json::Value above, but
    // now we are asking it for a Person as output.
    let p: Person = serde_json::from_str(data)?;

    // Do things just like with any other Rust data structure.
    println!(
        "Please call {} at the number {:?}",
        p.name, p.phones
    );

    let s = serde_json::to_string(&p)?;
    println!("{}", s);

    Ok(())
}
