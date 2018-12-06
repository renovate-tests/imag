## Mails {#sec:modules:mails}

---

**NOTE:** This is mostly a todo-list for the `imag-mail` command. Nothing shown
here is implemented. This "documentation-to-be" should be moved to
`imag-mail --help` eventually.
This list might be incomplete, details might be not possible to implement in the
way described or other dragons.

**Target audience:** People who want to implement `imag-mail`.

---

The Mails module implements a commandline email client. Emails can be written
(via `$EDITOR`) and viewed, also in threads. Emails can be crawled for creating
new contacts.

A Text User Interface is not planned, but might be there at some point.

The mail module implements a minimal Email client. It does not handle IMAP
syncing or SMTP things, it is just a _viewer_ for emails (a MUA).

The goal of the initial implementation is only a CLI, not a TUI like mutt
offers, for example (but that might be implemented later). As this is an imag
module, it also creates references to mails inside the imag store which can be
used by other tools then (for example `imag-link` to link an entry with a mail -
or the imag entry representing that mail).

So this module offers functionality to read (Maildir) mailboxes, search for and
list mails and mail-threads and reply to mails (by spawning the `$EDITOR`).

Outgoing mails are pushed to a special directory and can later on be send via
`imag-mail` which calls a MTA (for example msmtp) and also creates store entries
for the outgoing mails.


### Configuration

The following configuration variables are available for the imag-mail command:

* `mail.defaultaccount`: The name of the default account to use if the
  commandline parameters do not specify which account to use. The name must be
  in the `mail.accounts` array.
* `mail.accounts`:
  An array of account configuration. Each element in the array is a table of the
  following key-value pairs:
  * `name`: the name of the account. Names must be unique. Required.
  * `outgoingbox`: Path to mailbox to use for outgoing email. Required.
  * `draftbox`: Path to mailbox to use for outgoing email. Required.
  * `sentbox`: Path to mailbox to use for sent email. Required.
  ` `maildirroot`: Path to folder where all mailboxes for this account are
  located. Required.
  * `fetchcommand`: What commandline to invoke for fetching mails for this
    account. Optional - if not used, the global `mail.fetchcommand` will be
    used.
  * `postfetchcommand`: What commandline to invoke after fetching mails for this
    account. Optional - if not used, the global `mail.postfetchcommand` will be
    used.
  * `sendcommand`: What commandline to invoke for sending mails for this
    account. Optional - if not used, the global `mail.sendcommand` will be used.
  * `postsendcommand`: What commandline to invoke after sending mails for this
    account. Optional - if not used, the global `mail.postsendcommand` will be
    used.
* `mail.fetchcommand`: Command to use for fetching mail if no account-specific
  command was specified
  Available variables:
    * `{{accountname}}` - name of the account to fetch mail for.
    * `{{boxes}}` - a list of maildir paths to the boxes to fetch email for.
      imag provides primitives to transform this array.
      An example configuration for fetching with `offlineimap` might look like
      this: `offlineimap -a {{accountname}} -f {{concatsep "," (replace "/home/user/mails/" "" boxes)}}`
      to concatenate all boxes with a comma after removing a prefix.
      For a complete list of transformation functions, the `--help` flag shall
      be consulted.
      For more complicated transformations a bash/ruby/python script might be
      appropriate.
* `mail.postfetchcommand`: Command to use after fetching mail if no
  account-specific command was specified
  Available variables: Same as `mail.fetchcommand`.
* `mail.postsendcommand`: Command to use after sending mail if no
  account-specific command was specified
  Available variables: Same as `mail.sendcommand`.
* `mail.sendcommand`: Command to use for sending mail if no account-specific
  command was specified
    * `{{accountname}}` - name of the account to fetch mail for.
    * `{{mailfile}}` - The path of the mail to send


### CLI

The CLI of the imag-mail module is planned as follows:

* imag mail

    -A, --account   - Specify the "account" to use for the opperation by name.
                      If none is specified, the configuration is searched for a
                      default command.

* imag mail track <path> [opts...]
  Track a new mail, mail file passed as path

* imag mail scan <path> [opts...]
  Scan a maildir and track all untracked mails

    --refind        - re-find messages. Loads all messages which are known to imag
                      and compares identifiers, to update the imag-internal cache if
                      a mail got moved.
                      Without this flag, a modified email file might be added to
                      the imag store again, even if there's another entry in the
                      imag store refering to the same file.

* imag mail list <args...>
  List mails in a given mailbox for a given account or the default account

    -S, --style     - print messages in a certain style
                      Available:
                        - 'linewise'
                        - 'thread'

    -g, --grep      - Filter by grepping for a pattern in body and subject

    -d, --daterange - Filter by date(range)

    -F, --filter    - Filter by passed filter

        --thread    - Print only messages from the same thread as the found ones

    --format=<fmt>  - Format mails for showing.
                      --format always colorizes output (specify color in config)
                      except when using --no-pager or piping output.

                      When --tree is passed, the format is applied to the
                      fragment _after_ the tree graphic.

                      Default mode is 'default'.

                      Modes:
                        - 'subject': <Subject>
                        - 'simple': <From>: <Subject>
                        - 'default': <Date> - <From>: <Subject>
                        - 'fmt:<fmt>' format with passed format

                      Additional formats can be specified via the configuration
                      file. If a format has the same name as a predefined one,
                      the config overrides the predefined formats.

    --color         - Colorize output (default).
    --no-color      - Do never colorize output.

* imag mail show <args...>
  Show mail(s) - either in pager or by printing them to stdout.

    Mails are specified by message id or imag entry

    --refind        - If a imag entry is passed but the mail file is not there,
                      try to re-find it.

    --refind-in     - Same as --refind, but a path to a Maildir or a tree of
                      Maildirs might be passed to narrow down search space.

    -C, --concat    - Open all mails in one pager (by concatenating them)
                      instead of one pager per message.

    --pager         - Use pager to show mails (default).

    --no-pager      - Do not use pager to show mails.

    --multipager    - Pass all mails as arguments to one pager call instead of
                      calling the pager on each mail individually (default).
                      Only possible with --pager.

    --no-multipager - Disable --multipager.
                      Only possible with --pager.

    --format=<fmt>  - Format mails for showing.
                      --format always colorizes emails (specify color in config)
                      except when using --no-pager or piping output.

                      Modes:
                        - 'simple': Remove headers, except
                            From, To, Cc, Subject, Date,
                            Message-Id/References/In-Reply-To
                        - 'simple-imag': Same as 'simple' but also show imag
                          entry id.
                        - 'print': Show everything
                        - 'full': Show everything and add imag entry id
                        - 'minimal': Remove headers, except From, To, Cc, Subject, Date,
                        - 'tiny': Remove headers, except From, To, Subject
                        - 'fmt:<fmt>' format with passed format

                      Additional formats can be specified via the configuration
                      file. If a format has the same name as a predefined one,
                      the config overrides the predefined formats.

    --no-format     - Disable all formatting (same as --pretty=print and
                      disabling color output).

    --color         - Colorize output (default).
    --no-color      - Do never colorize output.

* imag mail new <args...>
  Craft a new mail and safe it in the <outgoing> folder

  Requires configuration:
    * mail.accounts.[.draftbox]
    * mail.accounts.[.outgoingbox]

        --outbox    - Specify the outbox for where the new mail should be stored
                      in, if it is not given in the config (or to override it)

        --to        - Specify to whom to send.
                      If the specified string does not contain a valid email
                      address, `imag contact find` is used to find the email
                      address (if not suppressed via --no-autofind).
                      Multiple allowed.

        --cc        - Specify to whom to send in CC.
                      If the specified string does not contain a valid email
                      address, `imag contact find` is used to find the email
                      address (if not suppressed via --no-autofind).
                      Multiple allowed.

        --bcc       - Specify to whom to send in BCC.
                      If the specified string does not contain a valid email
                      address, `imag contact find` is used to find the email
                      address (if not suppressed via --no-autofind).
                      Multiple allowed.

    --no-autofind   - Do not automatically find contacts
                      with `imag contact find`.

        --fcc       - Specify to store a copy of the mail somewhere.
                      Multiple allowed.

        --subject   - Specify subject.

        --gpg-sign  - Sign with gpg.

        --gpg-crypt - Crypt with gpg to all recipients.

        --no-track  - Do not track new mailfile with imag.

    -D, --draft     - Do not safe in "outgoing" box but rather in "draft" box.

* imag mail compose <args...>
  Same as 'new'.

* imag mail fetch <args...>
  Fetch emails

  Requires configuration:
    * mail.fetchcommand or mail.accounts[.fetchcommand]
    * mail.postfetchcommand or mail.accounts[.postfetchcommand] (optional)

    --all           - Fetch for all accounts
    --boxes         - Fetch only some boxes (does not work with --all)

* imag mail send <args...>
  Send emails from the outgoing folder, also move them to 'sent' boxes

  Requires configuration:
    * mail.accounts.[.outgoingbox]
    * mail.accounts.[.sentbox]
    * mail.sendcommand or mail.accounts[.sendcommand]
    * mail.postsendcommand or mail.accounts[.postsendcommand] (optional)

    --outbox        - Specify the outbox for where the mails that are about to
                      be send are stored in, if it is not given in the config
                      (or to override it).

    --sentbox       - Specify the sentbox for where the sent mails should be
                      moved after sending them, if it is not given in the config
                      (or to override it).

    --no-move-sent  - Do not move mail to the "sent" folder after sending it.

    --confirm       - Confirm each mail before sending (default).

    --no-confirm    - Do not confirm each mail before sending.

    --no-track      - Do not track mailfile with imag. Does only work if `imag
                      mail new` was invoked with `--no-track` (so that the mail
                      is not tracked already).

* imag mail mv <src mail> <dstbox>
  Move a mail to another mailbox

    --thread        - Move the complete thread of emails belonging to the
                      specified mail.

    --no-track      - Do not track new mailfile with imag. Does not work if
                      mailfile is already tracked with imag.

* imag mail find <args...>
  Search for a mail (by header field (msgid, from, to, cc, subject, date,
  date-range), body, ...)

    --msgid
    --no-msgid
    --from
    --no-from
    --to
    --no-to
    --cc
    --no-cc
    --subject
    --no-subject
    --date
    --no-date
    --body
    --no-body
    --daterange     - Toggle where to look at

    --print-entryid     - Print imag entry id when finding mail
    --no-print-entryid  - Do not print imag entry id when finding mail (default).

    --print=<what>  - What to print for the found mails.
                      Valid values:
                        - msgid
                        - subject
                        - from
                        - cc
                        - to
                        - date
                        - filepath (default)

* imag mail reply <args...>
  Reply to an email.

  Requires configuration: mail.accounts[.outgoingbox]

  Specify the mail to reply to by msgid, filepath or imag entry id.

    --add-to
    --add-cc
    --add-bcc       - Add another recipient. Multiple allowed.

    --no-track      - Do not track new mailfile with imag.

### Format specifiers

The `imag-mail` command supports formatting output automatically and via
predefined formats in the configuration file or by passing formatting
specifications via CLI.

The available formatting variables are:

* `H`: The complete message header as key-value-table
* `subject`: The subject of the message
* `date`: The date field of the message
* `body`: The body of the message
* `from`: The sender of the message
* `to`: The address of the receipient of message
* `fancyfromto`: The address of the sender of the message, or, if the sender was
  you, the receipient (prefixed with `F:` or `T:` respecively).

<!-- more might be defined -->

