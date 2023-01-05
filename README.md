# xcresult-json

A commmand-line utility to convert Xcode's proprietary xcresult bundle into a readable format for consumption outside of Apple's ecosystem.

# Usage
Assuming `/path/to/x.xcresult` is a valid path to your test results:
```sh
$ xcresult-json --input /path/to/x.xcresult convert --output /path/to/out
```

# Installation

```
$ brew tap malinskiy/tap
$ brew install xcresult-json
```

or you can manually download a package from [GitHub Releases](https://github.com/Malinskiy/xcresult-json/releases).

# Requirements
This tool is designed to run only within Apple's macOS environment.

# Design

While it's certainly possible to extract the same data from xcresult without access to macOS, potential breakage of the internal format makes this an unwise choice.

Underlying design of xcresult includes (as of Xcode 14) a [CAS](https://en.wikipedia.org/wiki/Content-addressable_storage) as well as a generic key-value pair meta information in the form of **Info.plist**.

`xcresult-json` is designed to be as minimally-invasive as possible to optimize with future versions of xcresult format. To convert the xcresult `xcresult-json` does three things:

1. Utilizes `xcresulttool` provided by Xcode distribution to extract entries from CAS
2. For structured objects `xcresulttool` exposes a JSON formatted output. If any references to other structured objects are found - `xcresult-json` will embed them into the same JSON to minimize the number of times you need to lookup entries in CAS.
3. For unstructured object (attachments of any sort like screenshots or diagnostic logs) raw binary is unpacked (currently the objects seems to be compressed using [Zstandard](http://facebook.github.io/zstd/)) using same `xcresulttool` and placed into a folder **cas** with the same reference id as the original xcresult report

Example output structure:
```sh
$ tree out/1017af07-cba3-4c79-ab87-d4c959bda03b
out/1017af07-cba3-4c79-ab87-d4c959bda03b
|-- cas
|   |-- 0~0xGbDwguplxVFNfSZgrqYIA4R_LMLZWpWxWXWRXXkQH2mRHGJikMyMrFB5VPYtUIggBKqU9v2IYZy90v7DVC2A==
|   |-- 0~3d6ZMvyj5amSQlg9L0WwjX_E6FsX_i74TAx2bb0TQfiIibjIFjkWZKskLsOMLgleNEzHK5Y1h2AAgLGw0rI0hw==
|   |-- 0~DO5sZbikU180sQSd1X3o-BsQSpXDL_jVDH__Yjvaw_djldyH8iTEyeFtJe9HoJb7El1mnDRnYZUUZSEmktbH5Q==
|   |-- 0~UhbINgK1oKc9IBEMsHAaeTIKRE0yONa_AdrNAiQkPcW2R9fpVi53wea9Woo_SVRG66ZFt-oXPI-ULdoopUQPNw==
|   |-- 0~c3www9rMSmVn4sPax9X7YeIVKp4mk9C2bSKNQco5dFMroJ5BhdsGJ14FeYsKNmC9e2DXNazef30OMPJFddiRog==
|   |-- 0~jOeQxW4ylp5Ax-ULmKd0CjOh2HlEKpIic-cYenNfQiJ0Lei5xoBtKFzlfArFhN_Cy8sGktNevO2qfOZxDGY-ZA==
|   `-- 0~uQWRV2bhbOXpQwbeX5Sh0ESxXKvsFnzN5HgBhDnqFWaFV6uQ6rAIZywjK7e6t6iisWdY94HzaS7kWJWSCS2CGw==
`-- xcresult.json

1 directory, 8 files

```

Where `xcresult.json` is the unwrapped structured object about the test run execution and the cas folder contains raw assets references by xcresult.json

# Help
```sh
$ xcresult-json convert --help
Usage: xcresult-json --input <INPUT> convert --output <OUTPUT>

Options:
  -o, --output <OUTPUT>  
  -h, --help             Print help information
```

# License
```
This program is free software; you can redistribute it and/or
modify it under the terms of the GNU General Public License
as published by the Free Software Foundation; either version 2
of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program; If not, see <http://www.gnu.org/licenses/>
```

