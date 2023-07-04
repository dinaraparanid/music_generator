# Music Generator

### Music generation app written on Rust

Application produces MIDI files with the lead melody
and a harmony. Generates both BPM (95..=115) and the lead melody.
For the lead melody, next algorithm is used:

1. Bar is separated onto 16 parts
2. For each position either note with length 1/16 
   of bar is putted, or skipped with pause.
3. Only single pause with 2/16 length is allowed
4. Pause with 3/16 and greater are not allowed.
5. Chosen notes are close to the key and lie on scale

Full report and project's description are attached as the report and the presentation.