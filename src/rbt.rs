use crate::bindings;
use roc_std::{RocList, RocStr};
use serde::de::Visitor;
use serde::ser::SerializeSeq;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Rbt {
    default: Job,
}

impl From<bindings::Rbt> for Rbt {
    fn from(rbt: bindings::Rbt) -> Self {
        let unwrapped = rbt.f0;

        Rbt {
            default: Job::from(unwrapped.default),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Job {
    command: Command,
    #[serde(
        serialize_with = "serialize_roc_list_of_roc_str",
        deserialize_with = "deserialize_roc_list_of_roc_str"
    )]
    inputFiles: RocList<RocStr>,
    #[serde(
        serialize_with = "serialize_roc_list_of_roc_str",
        deserialize_with = "deserialize_roc_list_of_roc_str"
    )]
    outputs: RocList<RocStr>,
}

impl From<bindings::Job> for Job {
    fn from(job: bindings::Job) -> Self {
        // let unwrapped = job.into_Job();
        let unwrapped = job.f0;

        Job {
            command: Command::from(unwrapped.command),
            inputFiles: unwrapped.inputFiles,
            outputs: unwrapped.outputs,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Command {
    tool: Tool,
    #[serde(
        serialize_with = "serialize_roc_list_of_roc_str",
        deserialize_with = "deserialize_roc_list_of_roc_str"
    )]
    args: RocList<RocStr>,
}

impl From<bindings::Command> for Command {
    fn from(command: bindings::Command) -> Self {
        // let unwrapped = command.into_Job();
        let unwrapped = command.f0;

        Command {
            tool: Tool::from(unwrapped.tool),
            args: unwrapped.args,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Tool {
    SystemTool {
        #[serde(
            serialize_with = "serialize_roc_str",
            deserialize_with = "deserialize_roc_str"
        )]
        name: RocStr,
    },
}

impl From<bindings::Tool> for Tool {
    fn from(tool: bindings::Tool) -> Self {
        Self::SystemTool { name: tool.f0 }
    }
}

// Remote Types
//// RocList<RocStr>

fn serialize_roc_list_of_roc_str<S>(
    list: &RocList<RocStr>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let mut seq = serializer.serialize_seq(Some(list.len()))?;
    for item in list {
        seq.serialize_element(item.as_str())?;
    }
    seq.end()
}

fn deserialize_roc_list_of_roc_str<'de, D>(deserializer: D) -> Result<RocList<RocStr>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    deserializer.deserialize_seq(RocListOfRocStringVisitor {})
}

struct RocListOfRocStringVisitor {}

impl<'de> Visitor<'de> for RocListOfRocStringVisitor {
    type Value = RocList<RocStr>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a list of strings")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let mut out: Vec<RocStr> = match seq.size_hint() {
            Some(hint) => Vec::with_capacity(hint),
            None => Vec::new(),
        };

        while let Some(next) = seq.next_element::<&str>()? {
            out.push(RocStr::from(next))
        }

        Ok(RocList::from_slice(&out))
    }
}

//// RocStr

fn serialize_roc_str<S>(roc_str: &RocStr, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(roc_str.as_str())
}

fn deserialize_roc_str<'de, D>(deserializer: D) -> Result<RocStr, D::Error>
where
    D: serde::Deserializer<'de>,
{
    deserializer.deserialize_string(RocStringVisitor {})
}

struct RocStringVisitor {}

impl<'de> Visitor<'de> for RocStringVisitor {
    type Value = RocStr;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a string")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(RocStr::from(value))
    }
}
