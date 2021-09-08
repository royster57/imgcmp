# imgcmp

A utility for determining whether two images are the same.
Two images are considered to be the same if one has been obtained from the other by a small edit, a resizing, a change of saturation, or a change of exposition.

---

## Usage
imgcmp IMG1_PATH IMG2_PATH

The uitility expects exactly two arguments. Providing more or fewer arguments than expected will result in an error.

Image formats are deduced from their file name extensions (".png", ".jpg", etc.). The formats of the two input images need not be the same.
However, passing an image file with no extension, or one with a mislabeled extension, will result in an error.

**Supported formats**: All formats supported by the Rust crate "image".

## Output
If the images are evaluated to be the same, the program will output the string "Pictures are the same" to stdout.
If the images are different, it will output "Pictures are different" to stdout.
If an error occurs, a description of the error will be written to stderr.

---

## Examples

### Example 1:  Resizing an image

Consider the following image of a cat, [cat.jpg](https://github.com/erubboli/img_cmp/blob/master/assets/cat.jpg):
![alt text](https://github.com/erubboli/img_cmp/blob/master/assets/cat.jpg "cat.jpg")

and suppose we have a resized version of cat.jpg, [cat_edited.jpg](https://github.com/erubboli/img_cmp/blob/master/assets/cat_edited.jpg)

Then running

`imgcmp cat.jpg cat_edited.jpg`

will output

`Pictures are the same`.

### Example 2: comparing a cat to a Ferrari

Let's compare our [cat.jpg](https://github.com/erubboli/img_cmp/blob/master/assets/cat.jpg):
with a [ferrari.jpg](https://github.com/erubboli/img_cmp/blob/master/assets/ferrari_roma.jpg):
![alt text](https://github.com/erubboli/img_cmp/blob/master/assets/ferrari_roma.jpg "ferrari_roma.jpg")

Running
`imgcmp cat.jpg ferrari.jpg` 

will output

`Pictures are different`.

### Example 3: changing the exposition

If we change the exposition of our
[ferrari.jpg](https://github.com/erubboli/img_cmp/blob/master/assets/ferrari_roma.jpg))
to obtain,
[ferrari_edited.png](https://github.com/erubboli/img_cmp/blob/master/assets/ferrari_roma_edited.png))
![alt text](https://github.com/erubboli/img_cmp/blob/master/assets/ferrari_roma_edited.png "ferrari_roma_edited.png")
Then running
`imgcmp ferrari.jpg ferrari_edited.png`

will yield

`Pictures are the same`.
# imgcmp
