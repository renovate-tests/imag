//
// imag - the personal information management suite for the commandline
// Copyright (C) 2015-2019 Matthias Beyer <mail@beyermatthias.de> and contributors
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License as published by the Free Software Foundation; version
// 2.1 of the License.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with this library; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA
//

use config::MailConfig;

pub struct MailSender<'a> {
    config: &'a MailConfig,
    account_name_to_send_with: Option<String>,

    rescan_maildirs: bool,
}

impl MailSender {
    pub fn new(config: &MailConfig) -> Self {
        MailSender {
            config,
            account_name_to_send_with: None,
            rescan_maildirs: false
        }
    }

    pub fn send_account(mut self, name: String) -> Self {
        self.account_name_to_send_with = Some(name);
        self
    }

    pub fn rescan_maildirs(mut self, b: bool) -> Self {
        self.rescan_maildirs = b;
        self
    }

    pub fn run(&self, store: &Store) -> Result<()> {
        let sendcommand = match self.account_name_to_send_with {
            Some(name) => self.config.sendcommand_for_account(name),
            None       => self.confnig.sendcommand(),
        };

        let postsendcommand = match self.account_name_to_send_with {
            Some(name) => self.config.postsendcommand_for_account(name),
            None       => self.confnig.sendcommand(),
        };

        let account = config
            .account(self.account_name_to_send_with)
            .ok_or_else(|| format_err!("Account '{}' does not exist", self.account_name_to_send_with))?;

        if sendcommand.contains(" ") {
            // error on whitespace in command
        }

        if postsendcommand.contains(" ") {
            // error on whitespace in command
        }

        // sendcommand
        //
        let outgoingbox = account
            .outgoingbox
            .to_str()
            .ok_or_else(|| format_err!("Cannot use '{:?}' as outgoingbox", account.outgoingbox))?;

        let mut output = Command::new(sendcommand)
            // TODO: Add argument support
            // TODO: Add support for passing config variables
            // TODO: Add support for passing environment
            .arg(outgoingbox)
            .wait_with_output()
            .context("Mail sending")?;

        write!(rt.stdout(), "{}", output.stdout)?;
        write!(rt.stderr(), "{}", output.stderr)?;

        // TODO: Move all files in outgoingbox to account.sentbox

        // postfetchcommand

        let output = Command::new(postsendcommand)
            // TODO: Add argument support
            // TODO: Add support for passing config variables
            .wait_with_output()
            .context("Post 'Mail sending' command")?;

        write!(rt.stdout(), "{}", output.stdout)?;
        write!(rt.stderr(), "{}", output.stderr)?;

        if self.rescan_maildirs {
            // scan
            // account.maildirroot
            // recursively for new mail and store them in imag
        }
    }

}


