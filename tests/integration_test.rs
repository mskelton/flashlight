use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use predicates::prelude::*;
use std::process::Command;

pub fn create_cmd(
    file: &assert_fs::NamedTempFile,
) -> Result<Command, Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("flashlight")?;
    cmd.current_dir(file.path().parent().unwrap());
    Ok(cmd)
}

#[test]
fn single_import() -> Result<(), Box<dyn std::error::Error>> {
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
fn multiple_imports() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("imports.tsx")?;
    file.write_str(
        "
        import { useState } from 'react'
        import { hi } from './yo'
        import { useMemo } from 'react'
        ",
    )?;

    let mut cmd = create_cmd(&file)?;
    cmd.arg("imports").arg("react");
    cmd.assert().success().stdout(predicate::str::diff(format!(
        "{}\n{}\n",
        "./imports.tsx:2:9 import { useState } from 'react'",
        "./imports.tsx:4:9 import { useMemo } from 'react'"
    )));

    Ok(())
}

#[test]
fn import_with_specifier() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("imports.tsx")?;
    file.write_str(
        "
        import { useState } from 'react'
        import { useMemo } from 'react'
        ",
    )?;

    let mut cmd = create_cmd(&file)?;
    cmd.arg("imports").arg("react").arg("useMemo");
    cmd.assert().success().stdout(predicate::str::diff(
        "./imports.tsx:3:9 import { useMemo } from 'react'\n",
    ));

    Ok(())
}

#[test]
fn tags() -> Result<(), Box<dyn std::error::Error>> {
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
fn namespaced_tags() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("tags.tsx")?;
    file.write_str(
        "
        const ui = <>
            <p class='foo'>hi</p>
            <foo.bar class='bar'>ho</foo.bar>
            <div class='foo'>bar</div>
        </>
        ",
    )?;

    let mut cmd = create_cmd(&file)?;
    cmd.arg("tags").arg("foo.bar");
    cmd.assert().success().stdout(predicate::str::diff(
        "./tags.tsx:4:13 <foo.bar class='bar'>ho</foo.bar>\n",
    ));

    Ok(())
}

#[test]
fn tags_by_attr() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("tags.tsx")?;
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
        "./tags.tsx:3:13 <p class='foo'>hi</p>",
        "./tags.tsx:6:17 <p class='bar'>ho</p>"
    )));

    Ok(())
}

#[test]
fn tags_by_attr_value_string() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("tags.tsx")?;
    file.write_str(
        "
        const ui = <>
            <p class='foo'>one</p>
            <p id='bar'>two</p>
            <div class='foo'>
                <p class='bar'>three</p>
            </div>
            <p class={\"bar\"}>four</p>
        </>
        ",
    )?;

    let mut cmd = create_cmd(&file)?;
    cmd.arg("tags").arg("p").arg("class=bar");
    cmd.assert().success().stdout(predicate::str::diff(format!(
        "{}\n{}\n",
        "./tags.tsx:6:17 <p class='bar'>three</p>",
        "./tags.tsx:8:13 <p class={\"bar\"}>four</p>",
    )));

    Ok(())
}

#[test]
fn tags_by_attr_value_number() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("tags.tsx")?;
    file.write_str(
        "
        const ui = <>
            <p class={1}>hi</p>
            <p id='bar'>ho</p>
            <div class='foo'>
                <p class={2}>ho</p>
            </div>
        </>
        ",
    )?;

    let mut cmd = create_cmd(&file)?;
    cmd.arg("tags").arg("p").arg("class=2");
    cmd.assert()
        .success()
        .stdout(predicate::str::diff("./tags.tsx:6:17 <p class={2}>ho</p>\n"));

    Ok(())
}

#[test]
fn tags_by_attr_value_bool() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("tags.tsx")?;
    file.write_str(
        "
        const ui = <>
            <p isRequired={false}>hi</p>
            <p id='bar'>ho</p>
            <p isRequired>cherry</p>
            <div class='foo'>
                <p isRequired={true}>o</p>
            </div>
        </>
        ",
    )?;

    let mut cmd = create_cmd(&file)?;
    cmd.arg("tags").arg("p").arg("isRequired=true");
    cmd.assert().success().stdout(predicate::str::diff(format!(
        "{}\n{}\n",
        "./tags.tsx:5:13 <p isRequired>cherry</p>",
        "./tags.tsx:7:17 <p isRequired={true}>o</p>",
    )));

    let mut cmd = create_cmd(&file)?;
    cmd.arg("tags").arg("p").arg("isRequired=false");
    cmd.assert().success().stdout(predicate::str::diff(
        "./tags.tsx:3:13 <p isRequired={false}>hi</p>\n",
    ));

    Ok(())
}

#[test]
fn tags_by_attr_value_regex() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("tags.tsx")?;
    file.write_str(
        "
        const ui = <>
            <p class={/a?.*cd/}>hi</p>
            <p id='bar'>ho</p>
            <div class='foo'>
                <p class={/abc/}>ho</p>
            </div>
        </>
        ",
    )?;

    let mut cmd = create_cmd(&file)?;
    cmd.arg("tags").arg("p").arg("class=a?.*cd");
    cmd.assert().success().stdout(predicate::str::diff(
        "./tags.tsx:3:13 <p class={/a?.*cd/}>hi</p>\n",
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

#[test]
fn searches_multiple_files() -> Result<(), Box<dyn std::error::Error>> {
    let dir = assert_fs::TempDir::new().unwrap();
    let foo = dir.child("foo.js");
    foo.write_str(
        "
        import {useState} from 'react'
        import {hi} from './yo'
        ",
    )?;

    let bar = dir.child("bar.tsx");
    bar.write_str(
        "
        import {createRoot} from 'react-dom'
        import {useMemo} from 'react'
        ",
    )?;

    let mut cmd = Command::cargo_bin("flashlight")?;
    cmd.current_dir(dir.path());
    cmd.arg("imports").arg("react");
    cmd.assert().success().stdout(predicate::str::diff(format!(
        "{}\n{}\n",
        "./foo.js:2:9 import {useState} from 'react'",
        "./bar.tsx:3:9 import {useMemo} from 'react'"
    )));

    dir.close().unwrap();
    Ok(())
}

#[test]
fn ignores_unsupported_file_types() -> Result<(), Box<dyn std::error::Error>> {
    let dir = assert_fs::TempDir::new().unwrap();
    let content = "import {useState} from 'react'";

    let foo = dir.child("foo.js");
    foo.write_str(content)?;
    let bar = dir.child("bar.vue");
    bar.write_str(content)?;
    let bar = dir.child("baz.txt");
    bar.write_str(content)?;

    let mut cmd = Command::cargo_bin("flashlight")?;
    cmd.current_dir(dir.path());
    cmd.arg("imports").arg("react");
    cmd.assert().success().stdout(predicate::str::diff(
        "./foo.js:1:1 import {useState} from 'react'\n",
    ));

    dir.close().unwrap();
    Ok(())
}

#[test]
fn supports_decorators() -> Result<(), Box<dyn std::error::Error>> {
    let dir = assert_fs::TempDir::new().unwrap();
    let content = "
        import {useState} from 'react'

        @foo
        class Foo{}
        ";

    let foo = dir.child("foo.js");
    foo.write_str(content)?;
    let bar = dir.child("bar.tsx");
    bar.write_str(content)?;

    let mut cmd = Command::cargo_bin("flashlight")?;
    cmd.current_dir(dir.path());
    cmd.arg("imports").arg("react");
    cmd.assert().success().stdout(predicate::str::diff(format!(
        "{}\n{}\n",
        "./foo.js:2:9 import {useState} from 'react'",
        "./bar.tsx:2:9 import {useState} from 'react'"
    )));

    dir.close().unwrap();
    Ok(())
}

#[test]
fn supports_import_assertions() -> Result<(), Box<dyn std::error::Error>> {
    let dir = assert_fs::TempDir::new().unwrap();
    let content = "
        import {useState} from 'react' with {type: 'json'}

        @foo
        class Foo{}
        ";

    let foo = dir.child("foo.js");
    foo.write_str(content)?;
    let bar = dir.child("bar.tsx");
    bar.write_str(content)?;

    let mut cmd = Command::cargo_bin("flashlight")?;
    cmd.current_dir(dir.path());
    cmd.arg("imports").arg("react");
    cmd.assert().success().stdout(predicate::str::diff(format!(
        "{}\n{}\n",
        "./foo.js:2:9 import {useState} from 'react' with {type: 'json'}",
        "./bar.tsx:2:9 import {useState} from 'react' with {type: 'json'}",
    )));

    dir.close().unwrap();
    Ok(())
}
