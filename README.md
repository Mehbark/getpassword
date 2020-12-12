# getpassword: a simple password file parser thing.
Remember to encrypt your password file to keep it safe, and then temporarily unencrypt it for use with this program.
Format for the password file is 'name, password, name, password, ' etc. The space is signficant.
Best used as part of an alias. For example:
`passwords = 'mcrypt -d ~/main/misc/passwords.nc; ~/main/programs/rustprograms/getpassword; rm ~/main/misc/passwords; sleep 15; clear;'`
