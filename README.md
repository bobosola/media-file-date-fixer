# mp4datecorrector

A common problem when copying MP4 files across devices is that one or both devices may clobber the "file modified" date and time of the copied file so that it becomes the date and time when the copy operation was done. This results in the natural shooting date order of the files being lost. This is very annoying should you wish to order your MP4 files by shooting date when editing in a NLE.

This application fixes the problem by amending the "file modified" date of all the MP4 files in a user-selected directory tree so that they match the "video encoded" date found inside the file's video metdata. This then enables the files to ordered by the original "file modfied" date in an NLE.
