# Changelog

## 2.0.1
* Now raising `errors::ErrorKind::MalformedLine` on all unexpected chars
* Ignoring empty and commented out lines

## 2.0.0
* Improved handling of IO errors
* Parsers now raise error when encountering format tag of different format
* Added this changelog
* Fixed error with malformed block header lines in life 1.05 files
* Added line number to `errors::ErrorKind::MalformedLine`
* Removed `errors::ErrorKind::InvalidRulesLine` in favor of `errors::ErrorKind::MalformedLine`
* Now correctly handling block offsets in 1.05 parser and cell coordinates in 1.06 parser that don't fit into an `i16`. Both will raise a `errors::ErrorKind::CoordinateOutOfRange`
* Life 1.05 parser now raises `MalformedLine` on unexpected characters inside blocks
* Life 1.05 parser now correctly handles coordinates that don't fit into an `i16`

## 1.0.0
* Input to parsers is now boxed, to make `Parser` a proper trait object

## 0.2.0
* Add parser for life 1.06 files
