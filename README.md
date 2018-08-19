Project Spacebar [![](https://travis-ci.org/LogoiLab/spacebar.svg?branch=master)](https://travis-ci.org/LogoiLab/spacebar)
===
An anti-plagiarism tool based on null width characters.

This tool hopes to eventually provide an entire suite of anti-plagiarism tools. Spacebar works by creating unique searchable tags that can be added to any text you create. These tags are invisible in most all editors and persist across copy-pasting which is where Spacebar gets its power. Spacebars can be added to blog posts, tweets, documents, spreadsheets, emails, even source code. Pretty much any electronic text can have a Spacebar added to it.

The hopes for this project is to provide an centralized database of Spacebar tags in a secure-searchable manner. Users will be able to create Spacebars and add them to their documents with ease while also managing who has used your content and where. Eventually a fully fledged web crawler may be added allowing for near instant notifications if your Spacebar is found on a website you haven't authorized.

Create a tag give it a name and description. It gets added to your clipboard. Paste it in any code or document either in comments or actual code (if your compiler doesn't care about whitespace characters).

If you find text or code later that you think is copied simply run the file through Project Spacebar and it will check your database to see if there is a spacebar associated with you.

This software is in an early development stage and has the current issues. (Those marked with a check may or may not have been fixed.):
- [X] Database may not de-duplicate properly.
- [x] Files that are one line(such as minified json) may fail to find spacebars if there are multiple spacebars.
- [x] On the creation of the default spacebar debug characters are printed.
- [ ] Calling unwrap somwhere when reading a file (option 5) causes a handled panic. It looks ugly but it doesn't break anything.
- [ ] Redirects aren't handled properly in the scraper. same HTTP errors. They are not ignored and are parsed as if they were the web page you wanted.
