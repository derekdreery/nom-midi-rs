mod note;
pub use note::Note;

#[derive(Debug, PartialEq, Clone)]
pub struct SimpleMidiFile<'src> {
    pub header: MidiHeader,
    pub tracks: Vec<Track<'src>>,
}

// header
// ======

/// A data structure for the Midi file header chunk
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct MidiHeader {
    /// The format of the file
    pub format: MidiFormat,
    /// The way time is divided
    pub division: Division,
}

/// Represents the different formats of a midi file
///
/// The parameter is the number of tracks
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum MidiFormat {
    /// There is only 1 track
    SingleTrack,
    /// There are multiple tracks to be played concurrently
    MultipleTrack(u16),
    /// There are multiple songs that can be played in any order/combination
    /// (e.g. a set of sequences for a drum machine)
    MultipleSong(u16),
}

impl MidiFormat {
    /// How many tracks are there in the format
    pub fn count(&self) -> u16 {
        match *self {
            MidiFormat::SingleTrack => 1,
            MidiFormat::MultipleTrack(ref n) => *n,
            MidiFormat::MultipleSong(ref n) => *n,
        }
    }
}

/// The way time is divided in the midi track
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Division {
    /// This means the time is indicated by the number of sub-divisons of a quarter-note
    /// (a.k.a. crotchet). For example, 4 would mean the maximum resolution is semi-quavers.
    ///
    /// A common choice is 96 = 2^5 * 3 allowing dotted notes down to hemidemisemiquavers
    Metrical(u16),
    /// The time is absolute, and expresses as a combination of frames per second, and the
    /// sub-frame resolution
    Timecode {
        /// The number of frames per second
        fps: Fps,
        /// The sub-frame resolution
        ///
        /// Typical values are 4 (corresponding to MIDI Time Code), 8, 10, 80 (corresponding to
        /// SMPTE bit resolution), or 100.
        res: u8,
    },
}

/// There are only 4 valid fps, below
#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(u8)]
pub enum Fps {
    /// 24 fps
    TwentyFour = 24,
    /// 25 fps
    TwentyFive = 25,
    /// 29 fps
    TwentyNine = 29,
    /// 30 fps
    Thirty = 30,
}

impl Into<u8> for Fps {
    fn into(self) -> u8 {
        self as u8
    }
}

// Track
// =====

/// A track chunk (a collection of events)
#[derive(Debug, PartialEq, Clone)]
pub struct Track<'src> {
    pub events: Vec<Event<'src>>,
}

// Events
// ======

/// An event present in a track chunk
#[derive(Debug, PartialEq, Clone)]
pub struct Event<'src> {
    pub delta_time: u32,
    pub event: EventType<'src>,
}

/// The type of an event in a track chunk, along with event-specific data
#[derive(Debug, PartialEq, Clone)]
pub enum EventType<'src> {
    Midi(MidiEvent),
    SystemExclusive(SystemExclusiveEvent<'src>),
    EscapeSequence(EscapeSequence<'src>),
    Meta(MetaEvent<'src>),
}

// Midi Events
// ===========

/// The midi event, along with the channel it applies to
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct MidiEvent {
    /// The channel the midi event applies to
    pub channel: u8,
    /// The event
    pub event: MidiEventType,
}

/// A midi event
///
/// Normally, the majority of messages will be of this type. They are the key messages for
/// starting and stopping sound, along with changing pitch.
///
/// Note that for all values, the top bit is not used, so the numbers will be interpreted the same
/// for either u8 or i8. I use u8 here.
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum MidiEventType {
    /// Stop sounding the given note
    ///
    /// The second param is the release velocity
    NoteOff(Note, u8),
    /// Start sounding the given note
    ///
    /// The second param is the attack velocity
    NoteOn(Note, u8),
    /// Apply aftertouch pressure to the given note
    ///
    /// The second param is the amount of aftertouch
    PolyphonicPressure(Note, u8),
    /// Set a controller to a value
    ///
    /// The first param is the controller to set, and the second param is the value to set it to
    Controller(u8, u8),
    /// Select the specified program
    ///
    /// The second param is the program to set.
    ProgramChange(u8),
    /// Allows all notes to have a specific aftertouch used as default, similar to
    /// `PolyphonicPressure`
    ChannelPressure(u8),
    /// Apply pitch bend to all notes
    ///
    /// First param is less significant byte, and second is most significant byte. The value of
    /// `0x00 0x40` means 'no bend', less means bend down and more means bend up.
    PitchBend(u8, u8),
}

// SysEx Events
// ============

/// A system exclusive message
#[derive(Debug, PartialEq, Clone)]
pub struct SystemExclusiveEvent<'src>(pub &'src [u8]);

/// An escape sequence (something not possible to include elsewhere)
#[derive(Debug, PartialEq, Clone)]
pub struct EscapeSequence<'src>(pub &'src [u8]);

// Meta Events
// ===========

/// A special non-MIDI event
#[derive(Debug, PartialEq, Clone)]
pub enum MetaEvent<'src> {
    /// The sequence number (as would be used in a MIDI Cue message)
    SequenceNumber(u16),
    /// Free text, can include comments and other useful information, if that information
    /// doesn't naturally fit in another text-based field
    Text(&'src [u8]),
    /// A copyright notice
    Copyright(&'src [u8]),
    /// The name of the current sequence or track (depending on context)
    SequenceOrTrackName(&'src [u8]),
    /// The name of the current track
    //TrackName(String),
    /// The name of the instrument for this track (e.g. "Flute", "Piano", "Tenor", etc.)
    InstrumentName(&'src [u8]),
    /// A syllable or set of syllables to be sung as part of a vocal track.
    Lyric(&'src [u8]),
    /// A useful position-dependent note in the music (e.g. rehersal mark "A", loop point,
    /// section name)
    Marker(&'src [u8]),
    /// A marker to indicate this event should be synchronized with some non-midi event, e.g. "car
    /// crash on screen", "actors leave stage", etc.
    CuePoint(&'src [u8]),
    /// Indicates what patch or program name should be used by the immediately subsequent Bank
    /// Select and Program Change messages.
    ProgramName(&'src [u8]),
    /// The name of the hardware device used to produce sounds for this track. Might be inserted
    /// for example if using a branded synth or keyboard to generate midi events.
    DeviceName(&'src [u8]),
    /// Indicate which channel subsequent SysEx and Meta events apply to. Lasts until the next
    /// event of this type, or a normal MIDI event
    MidiChannelPrefix(u8), // actually u4
    /// Specify which port future MIDI event apply to. This exists to increase the 4-bit channel
    /// limit, and so it's functionality overlaps with channels
    MidiPort(u8), // actually u7
    /// This event must be at the end of each track, and must not be anywhere else
    EndOfTrack,
    /// Specifies the number of microseconds per quarter note for future MIDI events.
    Tempo(u32), // actually u24
    /// This is complicated and I don't understand it.
    SMPTEOffset(SMPTEOffset),
    /// Set the time signature. If no time signature event occurs before a MIDI event the default
    /// is `(4, 4)`
    TimeSignature(TimeSignature),
    /// Set the key signature. The default is C major.
    KeySignature(KeySignature),
    /// Vendor specific events. I don't try to parse them - just return the data
    SequencerSpecificEvent(&'src [u8]),
    /// An unrecognised event. To be future-compatible, just ignore these
    Unknown(u8, &'src [u8]),
}

/// I don't understand this, but I should be decoding it correctly for those that do
#[derive(Debug, PartialEq, Clone)]
pub struct SMPTEOffset {
    pub fps: Fps,
    pub hour: u8,      // 0 - 23
    pub minute: u8,    // 0 - 59
    pub second: u8,    // 0 - 59
    pub no_frames: u8, // 0-23/24/28/29, depending on fps
    pub no_fractional_frames: u8,
}

/// A time signature
///
/// # Examples
/// Assuming `no_32nd_in_quarter` is 8
///
///  - A time signature of 4/4, with a metronome click every 1/4 note, would be encoded
///  `FF 58 04 04 02 18 08`. There are 24 MIDI Clocks per quarter-note, hence cc=24 (0x18).
///
///  - A time signature of 6/8, with a metronome click every 3rd 1/8 note, would be encoded
///  `FF 58 04 06 03 24 08` Remember, a 1/4 note is 24 MIDI Clocks, therefore a bar of 6/8 is
///  72 MIDI Clocks. Hence 3 1/8 notes is 36 (=0x24) MIDI Clocks.
///
/// (from http://www.somascape.org/midi/tech/mfile.html)
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct TimeSignature {
    /// The number of beats per bar
    pub top: u8,
    /// The size of those beats (1 = semibreve, 2 = minim, 3 = crotchet etc.)
    pub bottom: u8,
    /// Clock ticks between metronome clicks
    pub ticks_per_metronome_click: u8,
    /// The number of notated 32nd-notes in a MIDI quarter note - for a 1-1 corresponence this
    /// should be 8.
    pub number_32nd_in_quarter: u8,
}

/// All possible Key Signatures
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum KeySignature {
    CMajor,
    // sharps
    GMajor,
    DMajor,
    AMajor,
    EMajor,
    BMajor,
    FSharpMajor,
    CSharpMajor,
    // flats
    FMajor,
    BFlatMajor,
    EFlatMajor,
    AFlatMajor,
    DFlatMajor,
    GFlatMajor,
    CFlatMajor,

    // minor
    AMinor,
    // sharps
    EMinor,
    BMinor,
    FSharpMinor,
    CSharpMinor,
    GSharpMinor,
    DSharpMinor,
    ASharpMinor,
    // flats
    DMinor,
    GMinor,
    CMinor,
    FMinor,
    BFlatMinor,
    EFlatMinor,
    AFlatMinor,
}

impl KeySignature {
    /// Count the number of sharps or flats
    pub fn count(&self) -> u8 {
        use self::KeySignature::*;
        match *self {
            CMajor | AMinor => 0,
            GMajor | FMajor | EMinor | DMinor => 1,
            DMajor | BFlatMajor | BMinor | GMinor => 2,
            AMajor | EFlatMajor | FSharpMinor | CMinor => 3,
            EMajor | AFlatMajor | CSharpMinor | FMinor => 4,
            BMajor | DFlatMajor | GSharpMinor | BFlatMinor => 5,
            FSharpMajor | GFlatMajor | DSharpMinor | EFlatMinor => 6,
            CSharpMajor | CFlatMajor | ASharpMinor | AFlatMinor => 7,
        }
    }

    /// Helper fn for whether there are sharps or flats, that doesn't panic
    fn is_sharps_unchecked(&self) -> bool {
        use self::KeySignature::*;
        match *self {
            GMajor | DMajor | AMajor | EMajor | BMajor | FSharpMajor | CSharpMajor | EMinor
            | BMinor | FSharpMinor | CSharpMinor | GSharpMinor | DSharpMinor | ASharpMinor => true,
            _ => false,
        }
    }

    /// Whether there are sharps or flats
    ///
    /// # Panics
    /// Panics if there are no sharps or flats. Use `count` to check this first to avoid
    pub fn is_sharps(&self) -> bool {
        use self::KeySignature::*;
        match *self {
            CMajor | AMinor => panic!("No sharps or flats"),
            _ => self.is_sharps_unchecked(),
        }
    }

    /// Get a tuple of the number of sharps/flats, and a bool that is true for sharps, false for
    /// flats.
    ///
    /// The second value is not specified (could be anything) when the first is 0.
    pub fn for_display(&self) -> (u8, bool) {
        (self.count(), self.is_sharps_unchecked())
    }
}
