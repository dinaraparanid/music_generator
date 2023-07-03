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

### Generated results

[Example with D# key](D#-2023-07-02_15:40:47.519650174_+03:00.mid)

[Example with E key](E-2023-07-02_15:51:41.962661309_+03:00.mid)

[Example with F# key](F#-2023-07-02_23:02:59.680187162_+03:00.mid)