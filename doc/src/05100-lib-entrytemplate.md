## libimagentrytemplate

Helper library for the template pattern in imag. The template pattern is the
following:

* There is a kind of imag entry which functions as "template" for other entries.
* Instances are usages of a template

For example, a training session can be predefined by a template
"TrainingSession", where the fields "starttime", "endtime" and "kind" is
required to be filled for an instance.
If a user now has a training session, she can enter the data into imag, hence
creating an instance of the template.

This pattern is supported by using this library.

