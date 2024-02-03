[![crates-badge]][crates]\n[![docs-badge]][docs]\n![license-badge]\n\n# pico-detect\n\nThis library is a reimplementation of _Pixel Intensity Comparison-based Object_ (PICO) detection algorithms in Rust:\n\n- `Detector`: Cascade of binary classifiers from [pico];\n- `Localizer`: Localization with an ensemble of randomized trees from [picojs](https://github.com/nenadmarkus/picojs) (see `lploc.js`);\n- `Shaper`: Alignment with an ensemble of regression trees from [dlib](https://github.com/davisking/dlib) (see `shape_predictor`).\n\n## Example\n\nTo run CLI example, which takes an image, finds all faces, detects some landmarks and pupils:\n\n> **NOTE**: [Git LFS](https://git-lfs.github.com/) is needed to resolve binary files with `git clone`.\n>\n> If you don't want to use Git LFS you can download models (and test image) direct from this repo\n> (see **model** column in the table below)\n> and put them under [`models/`](./models) directory.\n\n```sh\ncargo run --release --example detect-faces -- --models-dir models -i "assets/test.png" --score 35.0 -o result.png\n```\n\nOutput image `result.png` should be like thi