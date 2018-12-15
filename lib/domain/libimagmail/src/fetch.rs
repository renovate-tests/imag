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

pub struct MailFetcher<'a> {
    config: &'a MailConfig,
    account_name_to_fetch: Option<String>,
    boxes: Vec<String>,

    rescan_maildirs: bool,
}

impl MailFetcher {
    pub fn new(config: &MailConfig) -> Self {
        MailFetcher {
            config,
            account_name_to_fetch: None,
            rescan_maildirs: false
        }
    }

    pub fn fetch_account(mut self, name: String) -> Self {
        self.account_name_to_fetch = Some(name);
        self
    }

    pub fn fetch_box(mut self, name: String) -> Self {
        self.boxes.push(name);
        self
    }

    pub fn fetch_boxes<I>(mut self, names: I) -> Self
        where I: IntoIterator<Item = String>
    {
        self.boxes.append(names.into_iter().collect())
        self
    }

    pub fn rescan_maildirs(mut self, b: bool) -> Self {
        self.rescan_maildirs = b;
        self
    }

    pub fn run(&self, store: &Store) -> Result<()> {
        let fetchcommand = match self.account_name_to_fetch {
            Some(name) => self.config.fetchcommand_for_account(name),
            None       => self.confnig.fetchcommand(),
        };

        let postfetchcommand = match self.account_name_to_fetch {
            Some(name) => self.config.postfetchcommand_for_account(name),
            None       => self.confnig.postfetchcommand(),
        };

        let account = config
            .account(self.account_name_to_fetch)
            .ok_or_else(|| format_err!("Account '{}' does not exist", self.account_name_to_fetch))?;

        if fetchcommand.contains(" ") {
            // error on whitespace in command
        }

        if postfetchcommand.contains(" ") {
            // error on whitespace in command
        }

        // fetchcommand

        let mut output = Command::new(fetchcommand)
            // TODO: Add argument support
            // TODO: Add support for passing config variables
            // TODO: Add support for passing environment
            .args(self.boxes)
            .wait_with_output()
            .context("Mail fetching")?;

        write!(rt.stdout(), "{}", output.stdout)?;
        write!(rt.stderr(), "{}", output.stderr)?;

        // postfetchcommand

        let output = Command::new(postfetchcommand)
            // TODO: Add argument support
            // TODO: Add support for passing config variables
            .wait_with_output()
            .context("Post 'Mail fetching' command")?;

        write!(rt.stdout(), "{}", output.stdout)?;
        write!(rt.stderr(), "{}", output.stderr)?;

        if self.rescan_maildirs {
            // scan
            // account.maildirroot
            // recursively for new mail and store them in imag
        }
    }

}


