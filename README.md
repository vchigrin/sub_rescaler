# sub_rescaler - Subtitles (.srt) files timestamp editing utility

## Motivation
Primary use case - is adjusting timings in one `.srt` file based on another, where timings is good.

This may be useful if you have `.srt` file with subtitles in desired language, but with "slightly wrong" timings.

## Usage
### Adjusting timestamps based on reference file.
Assume you have file files `movie.en.srt` with perfectly fitted English subtitles, and `movie.ru.srt` with subtitles in Russian, but skewed timings.
You can start with command
```
sub_rescaler -i movie.ru.srt -o movie_new.ru.srt sync-with movie.en.srt
```
to get `movie_new.ru.srt` file with subtitles in Russian, that has timings based on English subtitles.

In former example program assumes that first frame in Russian track must be synced with first in English, and the same for last frames.
Timings for intermediate frames will be interpolated based on this assumption.

This might be not the case if one of the files has extra frames at the beginning or at the end (or if one of them assembled for version of the movie with some scenes cut off).
To address this issue you can add "alignment points" - a pair of frames, for which you know that can be synchronized. Frame numbers here refer to numbers in the `.srt` file (this is just text file, can be viewed with any text editor).
In that case program will sync individually intervals from the beginning of the file to the first alignment point, then from the first to second, etc., then from the last to end of file.
Here is example command for this scenario:

```
sub_rescaler -i movie.ru.srt -o movie_new.ru.srt sync-with movie.en.srt -a 2:3 -a 30:39
```
It will rescale timestamps assuming source frame 2 in file with Russian subtitles correspond to frame 3 in reference frame with English subtitles; and source frame 30 to reference frame 39.

### Adding offset to subtitle items.
If for any reason you need to add/substract some duration from each subtitle item, you can do this with command like this:
```
sub_rescaler -i movie.srt -o movie_new.srt offset 2000
```
this will add 2 seconds (2000 milliseconds) to each item in file.
Command
```
sub_rescaler -i movie.srt -o movie_new.srt offset -- -2000
```
will do the opposite - substract 2 seconds from each subtitle item timestamp. (Note that this operation may cause some items to disappear if after adjusting they will get negative timestamps).

## Building.
To build this program you need [Rust toolchain](https://www.rust-lang.org/tools/install).
If you need pre-built binaries for any platform - feel free to make Issue to this project :)
