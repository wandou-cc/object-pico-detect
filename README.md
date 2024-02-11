[![crates-badge]][crates]\n[![docs-badge]][docs]\n![license-badge]\n\n# pico-detect\n\nThis library is a reimplementation of _Pixel Intensity Comparison-based Object_ (PICO) detection algorithms in Rust:\n\n- `Detector`: Cascade of binary classifiers from [pico];\n- `Localizer`: Localization with an ensemble of randomized trees from [picojs](https://github.com/nenadmarkus/picojs) (see `lploc.js`);\n- `Shaper`: Alignment with an ensemble of regression trees from [dlib](https://github.com/davisking/dlib) (see `shape_predictor`).\n\n## Example\n\nTo run CLI example, which takes an image, finds all faces, detects some landmarks and pupils:\n\n> **NOTE**: [Git LFS](https://git-lfs.github.com/) is needed to resolve binary files with `git clone`.\n>\n> If you don't want to use Git LFS you can download models (and test image) direct from this repo\n> (see **model** column in the table below)\n> and put them under [`models/`](./models) directory.\n\n```sh\ncargo run --release --example detect-faces -- --models-dir models -i "assets/test.png" --score 35.0 -o result.png\n```\n\nOutput image `result.png` should be like this:\n\n![visualization example](./assets/result.png)\n\n## Models\n\nEach algorithm requires to be loaded with correspondent binary model.\n\n| model                     | algorithm   | source                             | Description               |\n|---------------------------|-------------|------------------------------------|---------------------------|\n| [face.detector.bin]       | `Detector`  | [pico]                             | Human face classifier     |\n| [pupil.localizer.bin]     | `Localizer` | [puploc]                           | Human eye pupil localizer |\n| [face-5.shaper.bin]       | `Shaper`    | [shape_predictor_5_face_landmarks] | Human 5 face landmarks    |\n\n## References\n\n1. [N. Markus, M. Frljak, I. S. Pandzic, J. Ahlberg and R. Forchheimer, "Object Detection with Pixel Intensity Comparisons Organized in Decision Trees"](http://arxiv.org/abs/1305.4537)\n\n2. [Eye pupil localization with an ensemble of randomized trees](https://across.fer.hr/_download/repository/PR4885.pdf)\n\n3. [One Millisecond Face Alignment with an Ensemble of Regression Trees](https://www.cv-foundation.org/openaccess/content_cvpr_2014/papers/Kazemi_One_Millisecond_Face_2014_CVPR_paper.pdf)\n\n[crates]: https://crates.io/crates/pico-detect\n[docs]: https://docs.rs/pico-detect\n[docs-badge]: https://docs.rs/pico-detect/badge.svg\n[crates-badge]: https://img.shields.io/crates/v/pico-detect\n[license-badge]: https://img.shields.io/crates/l/pico-detect\n\n[pico]: https://github.com/nenadmarkus/pico\n\n[face.detector.bin]: https://github.com/wandou-cc/pico-detect/raw/master/models/face.detector.bin\n[pupil.localizer.bin]: https://github.com/wandou-cc/pico-detect/raw/master/models/pupil.localizer.bin\n[face-5.shaper.bin]: https://github.com/wandou-cc/pico-detect/raw/master/models/face-5.shaper.bin\n\n[puploc]: https://drone.nenadmarkus.com/data/blog-stuff/puploc.bin\n[shape_predictor_5_face_landmarks]: https://github.com/davisking/dlib-models#shape_predictor_5_face_landmarksdatbz2