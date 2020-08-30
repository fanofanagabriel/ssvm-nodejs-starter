const express = require('express');
var bodyParser = require('body-parser');
const multer = require('multer');
const upload = multer({ dest: 'uploads/' });
const sizeOf = require('image-size');
const exphbs = require('express-handlebars');
var jimp = require("jimp")
const fs = require('fs');
var mime = require('mime-types')

const { horizontally_flip } = require('../pkg/ssvm_nodejs_starter_lib.js');
const { vertically_flip } = require('../pkg/ssvm_nodejs_starter_lib.js');


const app = express();
app.use(bodyParser.json());
app.use(bodyParser.urlencoded({ extended: true }));

app.use(express.static(__dirname + '/uploads'));

app.engine('.hbs', exphbs({ extname: '.hbs' }));
app.set('view engine', '.hbs');

app.get('/', (req, res) => {
  return res.render('index', { layout: false });
});

app.post('/upload', upload.single('file'), (req, res) => {
  console.log("something is coming up");
  if (!req.file.mimetype.startsWith('image/')) {
    return res.status(422).json({
      error: 'The uploaded file must be an image'
    });
  }

  //const dimensions = sizeOf(req.file.path);
  //
  //if ((dimensions.width < 640) || (dimensions.height < 480)) {
  //  return res.status(422).json({
  //    error :'The image must be at least 640 x 480px'
  //  });
  //}
  the_latest_img = req.file;
  return res.status(200).send(req.file);
});

app.post('/flip', (req, res) => {
  var file = req.body.saturday_night;
  var type = req.body.fliptype;
  console.log(file)
  var data = fs.readFileSync("uploads/" + file);
  console.log("input:");
  console.log(data);

  new jimp("uploads/" + file, function (err, image) {
    var w = image.bitmap.width;
    var h = image.bitmap.height;
    var bitmap_data = image.bitmap.data;
    console.log("bitmap:");
    console.log(bitmap_data);
    console.log(bitmap_data.length);
    console.log(w * h * 4);

    var uintc8 = new Uint8ClampedArray(w * h * 4);
    if (type == 1) {
      uintc8 = horizontally_flip(w, h, bitmap_data);
    }
    else if (type == 2) {
      uintc8 = vertically_flip(w, h, bitmap_data);
    }
    console.log(uintc8);

    image.bitmap.data = uintc8;
    image.write("uploads/" + file);



    return res.status(200).send(file);



  });






});

app.listen(8080, () => {
  console.log('Express server listening on port 8080');
});
