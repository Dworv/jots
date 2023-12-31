use std::{path::Path, sync::Arc};

use dirs::data_dir;

use crate::{error::Error, sidebar::NoteHeading};

pub async fn prep_data_dir() -> Result<(), Error> {
    let jots_dir = data_dir().ok_or(Error::NoDataDir)?.join("jots");
    for subdir in ["notes"] {
        tokio::fs::create_dir_all(jots_dir.join(subdir))
            .await
            .map_err(|e| Error::Fs(e.kind()))?;
    }
    tokio::fs::create_dir_all(&jots_dir)
        .await
        .map_err(|e| Error::Fs(e.kind()))?;
    Ok(())
}

pub async fn load_file(path: impl AsRef<Path>) -> Result<Arc<String>, Error> {
    tokio::fs::read_to_string(path)
        .await
        .map(Arc::new)
        .map_err(|e| Error::Fs(e.kind()))
}

pub async fn load_notes() -> Result<Vec<NoteHeading>, Error> {
    let notes_dir = data_dir().ok_or(Error::NoDataDir)?.join("jots/notes");
    let mut notes = vec![];
    let mut dir = tokio::fs::read_dir(notes_dir)
        .await
        .map_err(|e| Error::Fs(e.kind()))?;
    while let Some(entry) = dir.next_entry().await.map_err(|e| Error::Fs(e.kind()))? {
        let path = entry.path();
        let title = path.file_name().unwrap().to_string_lossy().to_string();
        notes.push(NoteHeading { title, path });
    }
    Ok(notes)
}
