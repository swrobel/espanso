/*
 * This file is part of espanso.
 *
 * Copyright (C) 2020 Federico Terzi
 *
 * espanso is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * espanso is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with espanso.  If not, see <https://www.gnu.org/licenses/>.
 */

use crate::clipboard::ClipboardManager;
use crate::extension::ExtensionResult;
use serde_yaml::Mapping;
use std::collections::HashMap;

pub struct ClipboardExtension {
    clipboard_manager: Box<dyn ClipboardManager>,
}

impl ClipboardExtension {
    pub fn new(clipboard_manager: Box<dyn ClipboardManager>) -> ClipboardExtension {
        ClipboardExtension { clipboard_manager }
    }
}

impl super::Extension for ClipboardExtension {
    fn name(&self) -> String {
        String::from("clipboard")
    }

    fn calculate(
        &self,
        _: &Mapping,
        _: &Vec<String>,
        _: &HashMap<String, ExtensionResult>,
    ) -> Option<ExtensionResult> {
        if let Some(clipboard) = self.clipboard_manager.get_clipboard() {
            Some(ExtensionResult::Single(clipboard))
        } else {
            None
        }
    }
}
