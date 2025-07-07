# Mac Utility Scripts #

``release.sh`` builds both Intel and Mac silicon (ARM64) executables then combines them into a Universal Binary.

``codesign.sh`` handles packaging the Universal Binary into a DMG file for distribution. It also:
* code-signs the executable and the DMG
* notarizes and staples the DMG

in order to satisfy Gatekeeper requirements for Macs running Catalina or later.

This requires membership of the Apple Developer Program and an Apple "Developer ID Application" certificate installed locally in the keychain. This set-up allows signing without having to embed the certificate details in the script. There are more details of how this all works in the script comments.
