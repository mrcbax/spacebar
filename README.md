Project Spacebar [![](https://travis-ci.org/LogoiLab/spacebar.svg?branch=master)](https://travis-ci.org/LogoiLab/spacebar)
===
An anti-plagiarism tool based on null width characters.

Create a tag give it a name and description. It gets added to your clipboard. Paste it in any code or document either in comments or actual code (if your compiler doesn't care about whitespace characters).

If you find text or code later that you think is copied simply run the file through Project Spacebar and it will check your database to see if there is a spacebar associated with you.

This software is in an early development stage and has the current issues:
- Database does not dedup. Sorry.
- Files that are one line(such as minified json) may fail to find spacebars if there are multiple spacebars.