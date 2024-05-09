#![allow(clippy::print_stdout)]

use anyhow::{bail, Context};
use mdbook_i18n_helpers::extract_messages;
use std::{fs, path::Path};

fn main() -> anyhow::Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    let [en_filename, xx_filename] = match args.as_slice() {
        [_, en_filename, xx_filename] => [en_filename, xx_filename],
        [prog_name, ..] => bail!("Usage: {prog_name} <en-foo.md> <xx-foo.md>"),
        [] => unreachable!(),
    };

    let en_file = fs::read_to_string(Path::new(en_filename))
        .with_context(|| format!("Could not read {:?}", &en_filename))?;
    // println!("en_file: {:?}", en_file);
    let en_messages = extract_messages(&en_file);
    // println!("en: {en_messages:#?}");
    println!("{en_filename}: {:#?}", en_messages.len());

    let xx_file = fs::read_to_string(Path::new(xx_filename))
        .with_context(|| format!("Could not read {:?}", &xx_filename))?;
    // println!("xx_file: {:?}", xx_file);
    let xx_messages = extract_messages(&xx_file);
    // println!("xx: {xx_messages:#?}");
    println!("{xx_filename}: {:#?}", xx_messages.len());

    // let mut map = HashMap::new();
    // for (k, v) in en_messages {
    //     map.entry(k).or_insert((None, None)).0 = Some(v);
    // }
    // for (k, v) in xx_messages {
    //     map.entry(k).or_insert((None, None)).1 = Some(v);
    // }

    // let mut result: Vec<_> = map.into_iter().map(|(k, (v1, v2))| (k, v1, v2)).collect();
    // result.sort_by(|a, b| a.0.cmp(&b.0));

    // let result2 = result
    //     .into_iter()
    //     .map(|(k, v1, v2)| (v1.unwrap_or("".into()), v2.unwrap_or("".into())))
    //     .collect::<Vec<_>>();
    // println!("{:#?}", result2);

    Ok(())
}
