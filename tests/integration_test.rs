use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use predicates::prelude::*;
use std::process::Command;

fn create_cmd(
    file: &assert_fs::NamedTempFile,
) -> Result<Command, Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("flashlight")?;
    cmd.current_dir(file.path().parent().unwrap());
    Ok(cmd)
}

#[test]
fn find_imports() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("imports.tsx")?;
    file.write_str(
        "
        import { useState } from 'react'
        import { hi } from './yo'
        ",
    )?;

    let mut cmd = create_cmd(&file)?;
    cmd.arg("imports").arg("react");
    cmd.assert().success().stdout(predicate::str::diff(
        "./imports.tsx:2:9 import { useState } from 'react'\n",
    ));

    Ok(())
}

#[test]
fn finds_imports_by_specifier() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("imports-by-specifier.tsx")?;
    file.write_str(
        "
        import { useState } from 'react'
        import { useMemo } from 'react'
        ",
    )?;

    let mut cmd = create_cmd(&file)?;
    cmd.arg("imports").arg("react").arg("useMemo");
    cmd.assert().success().stdout(predicate::str::diff(
        "./imports-by-specifier.tsx:3:9 import { useMemo } from 'react'\n",
    ));

    Ok(())
}

#[test]
fn find_tags() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("tags.tsx")?;
    file.write_str(
        "
        const ui = <>
            <p class='foo'>hi</p>
            <p class='bar'>ho</p>
            <div class='foo'>bar</div>
        </>
        ",
    )?;

    let mut cmd = create_cmd(&file)?;
    cmd.arg("tags").arg("p");
    cmd.assert().success().stdout(predicate::str::diff(format!(
        "{}\n{}\n",
        "./tags.tsx:3:13 <p class='foo'>hi</p>",
        "./tags.tsx:4:13 <p class='bar'>ho</p>",
    )));

    Ok(())
}

#[test]
fn finds_tags_by_attr() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("tags-by-attr.tsx")?;
    file.write_str(
        "
        const ui = <>
            <p class='foo'>hi</p>
            <p id='bar'>ho</p>
            <div class='foo'>
                <p class='bar'>ho</p>
            </div>
        </>
        ",
    )?;

    let mut cmd = create_cmd(&file)?;
    cmd.arg("tags").arg("p").arg("class");
    cmd.assert().success().stdout(predicate::str::diff(format!(
        "{}\n{}\n",
        "./tags-by-attr.tsx:3:13 <p class='foo'>hi</p>",
        "./tags-by-attr.tsx:6:17 <p class='bar'>ho</p>"
    )));

    Ok(())
}

#[test]
fn finds_tags_by_attr_value() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("tags-by-attr-value.tsx")?;
    file.write_str(
        "
        const ui = <>
            <p class='foo'>hi</p>
            <p id='bar'>ho</p>
            <div class='foo'>
                <p class='bar'>ho</p>
            </div>
        </>
        ",
    )?;

    let mut cmd = create_cmd(&file)?;
    cmd.arg("tags").arg("p").arg("class=bar");
    cmd.assert().success().stdout(predicate::str::diff(
        "./tags-by-attr-value.tsx:6:17 <p class='bar'>ho</p>\n",
    ));

    Ok(())
}

#[test]
fn quickfix_logger() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("quickfix-logger.tsx")?;
    file.write_str(
        "
        const ui = <>
            <p class='foo'>hi</p>
            <p class='bar'>ho</p>
            <div class='foo'>bar</div>
        </>
        ",
    )?;

    let mut cmd = create_cmd(&file)?;
    cmd.arg("--format").arg("quickfix").arg("tags").arg("p");
    cmd.assert().success().stdout(predicate::str::contains(
        "/quickfix-logger.tsx:3:13: <p class='foo'>hi</p>",
    ));
    cmd.assert().success().stdout(predicate::str::contains(
        "/quickfix-logger.tsx:4:13: <p class='bar'>ho</p>",
    ));

    Ok(())
}

fn json(file: &str, line: u32, column: u32, text: &str) -> String {
    format!(
        "\\{{\"file\": \".*/{}\", \"line\": {}, \"column\": {}, \"text\": \"{}\"\\}}",
        file, line, column, text
    )
}

#[test]
fn json_logger() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("json-logger.tsx")?;
    file.write_str(
        "
        const ui = <>
            <p class='foo'>hi</p>
            <p class='bar'>ho</p>
            <div class='foo'>bar</div>
        </>
        ",
    )?;

    let mut cmd = create_cmd(&file)?;
    cmd.arg("--format").arg("json").arg("tags").arg("p");
    cmd.assert().success().stdout(
        predicate::str::is_match(
            json("json-logger.tsx", 3, 13, "<p class='foo'>hi</p>").as_str(),
        )
        .unwrap(),
    );

    cmd.assert().success().stdout(
        predicate::str::is_match(
            json("json-logger.tsx", 4, 13, "<p class='bar'>ho</p>").as_str(),
        )
        .unwrap(),
    );

    Ok(())
}
