pub fn welcome() {
  let logo: &'static str = r#"    
      .-----.
    .' -   - '.
   /  .-. .-.  \
   |  | | | |  |
    \ \o/ \o/ /
   _/    ^    \_
  | \  '---'  / |
  / /`--. .--`\ \
 / /'---` `---'\ \
 '.__.       .__.'
     `|     |`
      |     \
      \      '--.
       '.        `\
         `'---.   |
            ,__) /
             `..'

    FILLIPE'S GHOST DATABASE
                        "#;
  println!("{}", logo);
}
