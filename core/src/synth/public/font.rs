use crate::soundfont::SoundFontId;
use crate::synth::SoundFont;
use crate::synth::Synth;

impl Synth {
    /**
    Loads a SoundFont. The newly
    loaded SoundFont will be put on top of the SoundFont
    stack. Presets are searched starting from the SoundFont on the
    top of the stack, working the way down the stack until a preset
    is found.
     */
    pub fn add_font(&mut self, font: SoundFont, reset_presets: bool) -> SoundFontId {
        let id = self.fonts.insert(font);
        let id: SoundFontId = id.into();

        // Put SoundFont on top of the stack
        self.fonts_stack.insert(0, id);

        if reset_presets {
            self.program_reset();
        }

        id
    }

    /**
    Removes a SoundFont from the stack and deallocates it.
     */
    pub fn sfunload(&mut self, id: SoundFontId, reset_presets: bool) -> Result<(), ()> {
        let sfont = self.fonts.remove(id.0);
        self.fonts_stack.retain(|i| i == &id);

        if let Some(_) = sfont {
            if reset_presets {
                self.program_reset();
            } else {
                self.update_presets();
            }

            Ok(())
        } else {
            log::error!("No SoundFont with id = {:?}", id);

            Err(())
        }
    }

    /**
    Remove a SoundFont that was previously added using
    fluid_synth_add_sfont(). The synthesizer does not delete the
    SoundFont; this is responsability of the caller.
     */
    pub fn remove_sfont(&mut self, id: SoundFontId) {
        self.fonts.remove(id.0);
        self.fonts_stack.retain(|i| i == &id);

        self.remove_bank_offset(id);
        self.program_reset();
    }

    /**
    Count the number of loaded SoundFonts.
     */
    pub fn sfcount(&self) -> usize {
        self.fonts.len()
    }

    /**
    Get a SoundFont. The SoundFont is specified by its index on the
    stack. The top of the stack has index zero.

    - `num` The number of the SoundFont (0 <= num < sfcount)
     */
    pub fn get_nth_sfont(&self, num: usize) -> Option<&SoundFont> {
        // self.sfont.get(num)
        unimplemented!("get_sfont");
    }

    /**
    Get a SoundFont. The SoundFont is specified by its ID.
     */
    pub fn get_sfont(&self, id: SoundFontId) -> Option<&SoundFont> {
        self.fonts.get(id.0)
    }
}
