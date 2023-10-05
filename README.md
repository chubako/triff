#triff
##TRee dIFFerence

triff reports differences between directory A and directory B. Files and directories are compared by path, walking directory A using a depth-first search.

Main use cases are:

  1. remove duplicate files from A (provided they have the same relative paths in A and B)
  2. check full backups
  3. check recursive copies

Messages are logged assuming case 1, but no action is taken. so the report can be used as see fit.

triff reports:

  - if a file or directory exists in B but not in A:      it's reported as WARNING (and defined as an "outlier")
  - if a file or directory exists in A but not in B:      it's reported as ERROR
  - if a file path exists in A and B:
    - if file has different apparent size or real size:   it's reported as WARNING
    - if file has different permissions:                  it's reported as WARNING
    - if file has different owner:                        it's reported as WARNING
    - if file has different group:                        it's reported as WARNING
    - if file is non-regular:                             it's reported as WARNING
    - if file is regular, and has different content:      it's reported as ERROR
    - if all checks are ok for a regular file:            it's reported as SUCCESS
  - if a directory path exists in A and B:
    - every entry in the directory is checked
    - if there are no errors:                             it's reported as SUCCESS
    - if any error is detected:                           it's reported as ERROR

Every report (WARNING, ERROR or SUCCESS), is followed by a suggested ACTION:

  - for SUCCESS,  the action suggested it to remove the file in A (assuming we want to get rid of duplicated files)
  - for ERROR,
      - if file exists in both directories, the action suggested it to list the file in A and B
      - otherwise, the action suggested it to list the parent directory
  - for WARNINGS, the action suggested it to list the file in A and B

Sometimes INFO messages are provided (ie: when starting to scan a directory).

Messages are printed depending on the loglevel:

  1. ERROR
  2. WARNING
  3. ACTION
  4. SUCCESS
  5. INFO

Loglevel is controlled by the flag "-v LOGLEVEL_NUMBER" or "--verbosity LOGLEVEL_NUMBER".

Output can be sent to a logfile, specifing the flag "--logfile FILE_PATH".

Some preliminaries benchmarks were run, and performace seems similar to running "diff -rq".

Content check progress is shown (using [indicatif](https://crates.io/crates/indicatif)).

Other flags:

  - --onlypath: if relative file path exists in A and B report SUCCESS (without any additional test). This is useful to have a preliminary check to see if both directories are similar (ie: wh
  - --nocheckcontent: for regular files, skip checking content, reporting SUCCESS
  - --regular-info: if True, report non-regular files as INFO. Otherwise, report as WARNING (combined with verbosity, helps controlling how much information is reported)
  - --version
