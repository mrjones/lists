/**
 * @fileoverview
 * @enhanceable
 * @public
 */
// GENERATED CODE -- DO NOT EDIT!

var jspb = require('google-protobuf');
var goog = jspb;
var global = Function('return this')();

goog.exportSymbol('proto.LinkAnnotation', null, global);
goog.exportSymbol('proto.List', null, global);
goog.exportSymbol('proto.ListItem', null, global);
goog.exportSymbol('proto.StreetEasyAnnotation', null, global);
goog.exportSymbol('proto.TextAnnotation', null, global);
goog.exportSymbol('proto.User', null, global);

/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.User = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.User, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  proto.User.displayName = 'proto.User';
}


if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto suitable for use in Soy templates.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     com.google.apps.jspb.JsClassTemplate.JS_RESERVED_WORDS.
 * @param {boolean=} opt_includeInstance Whether to include the JSPB instance
 *     for transitional soy proto support: http://goto/soy-param-migration
 * @return {!Object}
 */
proto.User.prototype.toObject = function(opt_includeInstance) {
  return proto.User.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Whether to include the JSPB
 *     instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.User} msg The msg instance to transform.
 * @return {!Object}
 */
proto.User.toObject = function(includeInstance, msg) {
  var f, obj = {
    id: jspb.Message.getFieldWithDefault(msg, 1, 0),
    name: jspb.Message.getFieldWithDefault(msg, 2, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.User}
 */
proto.User.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.User;
  return proto.User.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.User} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.User}
 */
proto.User.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setId(value);
      break;
    case 2:
      var value = /** @type {string} */ (reader.readString());
      msg.setName(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Class method variant: serializes the given message to binary data
 * (in protobuf wire format), writing to the given BinaryWriter.
 * @param {!proto.User} message
 * @param {!jspb.BinaryWriter} writer
 */
proto.User.serializeBinaryToWriter = function(message, writer) {
  message.serializeBinaryToWriter(writer);
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.User.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  this.serializeBinaryToWriter(writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the message to binary data (in protobuf wire format),
 * writing to the given BinaryWriter.
 * @param {!jspb.BinaryWriter} writer
 */
proto.User.prototype.serializeBinaryToWriter = function (writer) {
  var f = undefined;
  f = this.getId();
  if (f !== 0) {
    writer.writeInt64(
      1,
      f
    );
  }
  f = this.getName();
  if (f.length > 0) {
    writer.writeString(
      2,
      f
    );
  }
};


/**
 * optional int64 id = 1;
 * @return {number}
 */
proto.User.prototype.getId = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 1, 0));
};


/** @param {number} value */
proto.User.prototype.setId = function(value) {
  jspb.Message.setField(this, 1, value);
};


/**
 * optional string name = 2;
 * @return {string}
 */
proto.User.prototype.getName = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/** @param {string} value */
proto.User.prototype.setName = function(value) {
  jspb.Message.setField(this, 2, value);
};



/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.List = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, proto.List.repeatedFields_, null);
};
goog.inherits(proto.List, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  proto.List.displayName = 'proto.List';
}
/**
 * List of repeated fields within this message type.
 * @private {!Array<number>}
 * @const
 */
proto.List.repeatedFields_ = [2];



if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto suitable for use in Soy templates.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     com.google.apps.jspb.JsClassTemplate.JS_RESERVED_WORDS.
 * @param {boolean=} opt_includeInstance Whether to include the JSPB instance
 *     for transitional soy proto support: http://goto/soy-param-migration
 * @return {!Object}
 */
proto.List.prototype.toObject = function(opt_includeInstance) {
  return proto.List.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Whether to include the JSPB
 *     instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.List} msg The msg instance to transform.
 * @return {!Object}
 */
proto.List.toObject = function(includeInstance, msg) {
  var f, obj = {
    name: jspb.Message.getFieldWithDefault(msg, 1, ""),
    itemsList: jspb.Message.toObjectList(msg.getItemsList(),
    proto.ListItem.toObject, includeInstance)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.List}
 */
proto.List.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.List;
  return proto.List.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.List} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.List}
 */
proto.List.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setName(value);
      break;
    case 2:
      var value = new proto.ListItem;
      reader.readMessage(value,proto.ListItem.deserializeBinaryFromReader);
      msg.addItems(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Class method variant: serializes the given message to binary data
 * (in protobuf wire format), writing to the given BinaryWriter.
 * @param {!proto.List} message
 * @param {!jspb.BinaryWriter} writer
 */
proto.List.serializeBinaryToWriter = function(message, writer) {
  message.serializeBinaryToWriter(writer);
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.List.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  this.serializeBinaryToWriter(writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the message to binary data (in protobuf wire format),
 * writing to the given BinaryWriter.
 * @param {!jspb.BinaryWriter} writer
 */
proto.List.prototype.serializeBinaryToWriter = function (writer) {
  var f = undefined;
  f = this.getName();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = this.getItemsList();
  if (f.length > 0) {
    writer.writeRepeatedMessage(
      2,
      f,
      proto.ListItem.serializeBinaryToWriter
    );
  }
};


/**
 * optional string name = 1;
 * @return {string}
 */
proto.List.prototype.getName = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/** @param {string} value */
proto.List.prototype.setName = function(value) {
  jspb.Message.setField(this, 1, value);
};


/**
 * repeated ListItem items = 2;
 * If you change this array by adding, removing or replacing elements, or if you
 * replace the array itself, then you must call the setter to update it.
 * @return {!Array.<!proto.ListItem>}
 */
proto.List.prototype.getItemsList = function() {
  return /** @type{!Array.<!proto.ListItem>} */ (
    jspb.Message.getRepeatedWrapperField(this, proto.ListItem, 2));
};


/** @param {!Array.<!proto.ListItem>} value */
proto.List.prototype.setItemsList = function(value) {
  jspb.Message.setRepeatedWrapperField(this, 2, value);
};


/**
 * @param {!proto.ListItem=} opt_value
 * @param {number=} opt_index
 * @return {!proto.ListItem}
 */
proto.List.prototype.addItems = function(opt_value, opt_index) {
  return jspb.Message.addToRepeatedWrapperField(this, 2, opt_value, proto.ListItem, opt_index);
};


proto.List.prototype.clearItemsList = function() {
  this.setItemsList([]);
};



/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.ListItem = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, proto.ListItem.repeatedFields_, null);
};
goog.inherits(proto.ListItem, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  proto.ListItem.displayName = 'proto.ListItem';
}
/**
 * List of repeated fields within this message type.
 * @private {!Array<number>}
 * @const
 */
proto.ListItem.repeatedFields_ = [4,5,6];



if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto suitable for use in Soy templates.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     com.google.apps.jspb.JsClassTemplate.JS_RESERVED_WORDS.
 * @param {boolean=} opt_includeInstance Whether to include the JSPB instance
 *     for transitional soy proto support: http://goto/soy-param-migration
 * @return {!Object}
 */
proto.ListItem.prototype.toObject = function(opt_includeInstance) {
  return proto.ListItem.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Whether to include the JSPB
 *     instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.ListItem} msg The msg instance to transform.
 * @return {!Object}
 */
proto.ListItem.toObject = function(includeInstance, msg) {
  var f, obj = {
    id: jspb.Message.getFieldWithDefault(msg, 1, 0),
    name: jspb.Message.getFieldWithDefault(msg, 2, ""),
    description: jspb.Message.getFieldWithDefault(msg, 3, ""),
    linkAnnotationsList: jspb.Message.toObjectList(msg.getLinkAnnotationsList(),
    proto.LinkAnnotation.toObject, includeInstance),
    textAnnotationsList: jspb.Message.toObjectList(msg.getTextAnnotationsList(),
    proto.TextAnnotation.toObject, includeInstance),
    streeteasyAnnotationsList: jspb.Message.toObjectList(msg.getStreeteasyAnnotationsList(),
    proto.StreetEasyAnnotation.toObject, includeInstance)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.ListItem}
 */
proto.ListItem.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.ListItem;
  return proto.ListItem.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.ListItem} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.ListItem}
 */
proto.ListItem.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setId(value);
      break;
    case 2:
      var value = /** @type {string} */ (reader.readString());
      msg.setName(value);
      break;
    case 3:
      var value = /** @type {string} */ (reader.readString());
      msg.setDescription(value);
      break;
    case 4:
      var value = new proto.LinkAnnotation;
      reader.readMessage(value,proto.LinkAnnotation.deserializeBinaryFromReader);
      msg.addLinkAnnotations(value);
      break;
    case 5:
      var value = new proto.TextAnnotation;
      reader.readMessage(value,proto.TextAnnotation.deserializeBinaryFromReader);
      msg.addTextAnnotations(value);
      break;
    case 6:
      var value = new proto.StreetEasyAnnotation;
      reader.readMessage(value,proto.StreetEasyAnnotation.deserializeBinaryFromReader);
      msg.addStreeteasyAnnotations(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Class method variant: serializes the given message to binary data
 * (in protobuf wire format), writing to the given BinaryWriter.
 * @param {!proto.ListItem} message
 * @param {!jspb.BinaryWriter} writer
 */
proto.ListItem.serializeBinaryToWriter = function(message, writer) {
  message.serializeBinaryToWriter(writer);
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.ListItem.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  this.serializeBinaryToWriter(writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the message to binary data (in protobuf wire format),
 * writing to the given BinaryWriter.
 * @param {!jspb.BinaryWriter} writer
 */
proto.ListItem.prototype.serializeBinaryToWriter = function (writer) {
  var f = undefined;
  f = this.getId();
  if (f !== 0) {
    writer.writeInt64(
      1,
      f
    );
  }
  f = this.getName();
  if (f.length > 0) {
    writer.writeString(
      2,
      f
    );
  }
  f = this.getDescription();
  if (f.length > 0) {
    writer.writeString(
      3,
      f
    );
  }
  f = this.getLinkAnnotationsList();
  if (f.length > 0) {
    writer.writeRepeatedMessage(
      4,
      f,
      proto.LinkAnnotation.serializeBinaryToWriter
    );
  }
  f = this.getTextAnnotationsList();
  if (f.length > 0) {
    writer.writeRepeatedMessage(
      5,
      f,
      proto.TextAnnotation.serializeBinaryToWriter
    );
  }
  f = this.getStreeteasyAnnotationsList();
  if (f.length > 0) {
    writer.writeRepeatedMessage(
      6,
      f,
      proto.StreetEasyAnnotation.serializeBinaryToWriter
    );
  }
};


/**
 * optional int64 id = 1;
 * @return {number}
 */
proto.ListItem.prototype.getId = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 1, 0));
};


/** @param {number} value */
proto.ListItem.prototype.setId = function(value) {
  jspb.Message.setField(this, 1, value);
};


/**
 * optional string name = 2;
 * @return {string}
 */
proto.ListItem.prototype.getName = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/** @param {string} value */
proto.ListItem.prototype.setName = function(value) {
  jspb.Message.setField(this, 2, value);
};


/**
 * optional string description = 3;
 * @return {string}
 */
proto.ListItem.prototype.getDescription = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 3, ""));
};


/** @param {string} value */
proto.ListItem.prototype.setDescription = function(value) {
  jspb.Message.setField(this, 3, value);
};


/**
 * repeated LinkAnnotation link_annotations = 4;
 * If you change this array by adding, removing or replacing elements, or if you
 * replace the array itself, then you must call the setter to update it.
 * @return {!Array.<!proto.LinkAnnotation>}
 */
proto.ListItem.prototype.getLinkAnnotationsList = function() {
  return /** @type{!Array.<!proto.LinkAnnotation>} */ (
    jspb.Message.getRepeatedWrapperField(this, proto.LinkAnnotation, 4));
};


/** @param {!Array.<!proto.LinkAnnotation>} value */
proto.ListItem.prototype.setLinkAnnotationsList = function(value) {
  jspb.Message.setRepeatedWrapperField(this, 4, value);
};


/**
 * @param {!proto.LinkAnnotation=} opt_value
 * @param {number=} opt_index
 * @return {!proto.LinkAnnotation}
 */
proto.ListItem.prototype.addLinkAnnotations = function(opt_value, opt_index) {
  return jspb.Message.addToRepeatedWrapperField(this, 4, opt_value, proto.LinkAnnotation, opt_index);
};


proto.ListItem.prototype.clearLinkAnnotationsList = function() {
  this.setLinkAnnotationsList([]);
};


/**
 * repeated TextAnnotation text_annotations = 5;
 * If you change this array by adding, removing or replacing elements, or if you
 * replace the array itself, then you must call the setter to update it.
 * @return {!Array.<!proto.TextAnnotation>}
 */
proto.ListItem.prototype.getTextAnnotationsList = function() {
  return /** @type{!Array.<!proto.TextAnnotation>} */ (
    jspb.Message.getRepeatedWrapperField(this, proto.TextAnnotation, 5));
};


/** @param {!Array.<!proto.TextAnnotation>} value */
proto.ListItem.prototype.setTextAnnotationsList = function(value) {
  jspb.Message.setRepeatedWrapperField(this, 5, value);
};


/**
 * @param {!proto.TextAnnotation=} opt_value
 * @param {number=} opt_index
 * @return {!proto.TextAnnotation}
 */
proto.ListItem.prototype.addTextAnnotations = function(opt_value, opt_index) {
  return jspb.Message.addToRepeatedWrapperField(this, 5, opt_value, proto.TextAnnotation, opt_index);
};


proto.ListItem.prototype.clearTextAnnotationsList = function() {
  this.setTextAnnotationsList([]);
};


/**
 * repeated StreetEasyAnnotation streeteasy_annotations = 6;
 * If you change this array by adding, removing or replacing elements, or if you
 * replace the array itself, then you must call the setter to update it.
 * @return {!Array.<!proto.StreetEasyAnnotation>}
 */
proto.ListItem.prototype.getStreeteasyAnnotationsList = function() {
  return /** @type{!Array.<!proto.StreetEasyAnnotation>} */ (
    jspb.Message.getRepeatedWrapperField(this, proto.StreetEasyAnnotation, 6));
};


/** @param {!Array.<!proto.StreetEasyAnnotation>} value */
proto.ListItem.prototype.setStreeteasyAnnotationsList = function(value) {
  jspb.Message.setRepeatedWrapperField(this, 6, value);
};


/**
 * @param {!proto.StreetEasyAnnotation=} opt_value
 * @param {number=} opt_index
 * @return {!proto.StreetEasyAnnotation}
 */
proto.ListItem.prototype.addStreeteasyAnnotations = function(opt_value, opt_index) {
  return jspb.Message.addToRepeatedWrapperField(this, 6, opt_value, proto.StreetEasyAnnotation, opt_index);
};


proto.ListItem.prototype.clearStreeteasyAnnotationsList = function() {
  this.setStreeteasyAnnotationsList([]);
};



/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.LinkAnnotation = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.LinkAnnotation, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  proto.LinkAnnotation.displayName = 'proto.LinkAnnotation';
}


if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto suitable for use in Soy templates.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     com.google.apps.jspb.JsClassTemplate.JS_RESERVED_WORDS.
 * @param {boolean=} opt_includeInstance Whether to include the JSPB instance
 *     for transitional soy proto support: http://goto/soy-param-migration
 * @return {!Object}
 */
proto.LinkAnnotation.prototype.toObject = function(opt_includeInstance) {
  return proto.LinkAnnotation.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Whether to include the JSPB
 *     instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.LinkAnnotation} msg The msg instance to transform.
 * @return {!Object}
 */
proto.LinkAnnotation.toObject = function(includeInstance, msg) {
  var f, obj = {
    id: jspb.Message.getFieldWithDefault(msg, 1, 0),
    url: jspb.Message.getFieldWithDefault(msg, 2, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.LinkAnnotation}
 */
proto.LinkAnnotation.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.LinkAnnotation;
  return proto.LinkAnnotation.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.LinkAnnotation} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.LinkAnnotation}
 */
proto.LinkAnnotation.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setId(value);
      break;
    case 2:
      var value = /** @type {string} */ (reader.readString());
      msg.setUrl(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Class method variant: serializes the given message to binary data
 * (in protobuf wire format), writing to the given BinaryWriter.
 * @param {!proto.LinkAnnotation} message
 * @param {!jspb.BinaryWriter} writer
 */
proto.LinkAnnotation.serializeBinaryToWriter = function(message, writer) {
  message.serializeBinaryToWriter(writer);
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.LinkAnnotation.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  this.serializeBinaryToWriter(writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the message to binary data (in protobuf wire format),
 * writing to the given BinaryWriter.
 * @param {!jspb.BinaryWriter} writer
 */
proto.LinkAnnotation.prototype.serializeBinaryToWriter = function (writer) {
  var f = undefined;
  f = this.getId();
  if (f !== 0) {
    writer.writeInt64(
      1,
      f
    );
  }
  f = this.getUrl();
  if (f.length > 0) {
    writer.writeString(
      2,
      f
    );
  }
};


/**
 * optional int64 id = 1;
 * @return {number}
 */
proto.LinkAnnotation.prototype.getId = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 1, 0));
};


/** @param {number} value */
proto.LinkAnnotation.prototype.setId = function(value) {
  jspb.Message.setField(this, 1, value);
};


/**
 * optional string url = 2;
 * @return {string}
 */
proto.LinkAnnotation.prototype.getUrl = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/** @param {string} value */
proto.LinkAnnotation.prototype.setUrl = function(value) {
  jspb.Message.setField(this, 2, value);
};



/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.TextAnnotation = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.TextAnnotation, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  proto.TextAnnotation.displayName = 'proto.TextAnnotation';
}


if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto suitable for use in Soy templates.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     com.google.apps.jspb.JsClassTemplate.JS_RESERVED_WORDS.
 * @param {boolean=} opt_includeInstance Whether to include the JSPB instance
 *     for transitional soy proto support: http://goto/soy-param-migration
 * @return {!Object}
 */
proto.TextAnnotation.prototype.toObject = function(opt_includeInstance) {
  return proto.TextAnnotation.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Whether to include the JSPB
 *     instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.TextAnnotation} msg The msg instance to transform.
 * @return {!Object}
 */
proto.TextAnnotation.toObject = function(includeInstance, msg) {
  var f, obj = {
    id: jspb.Message.getFieldWithDefault(msg, 1, 0),
    text: jspb.Message.getFieldWithDefault(msg, 2, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.TextAnnotation}
 */
proto.TextAnnotation.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.TextAnnotation;
  return proto.TextAnnotation.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.TextAnnotation} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.TextAnnotation}
 */
proto.TextAnnotation.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setId(value);
      break;
    case 2:
      var value = /** @type {string} */ (reader.readString());
      msg.setText(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Class method variant: serializes the given message to binary data
 * (in protobuf wire format), writing to the given BinaryWriter.
 * @param {!proto.TextAnnotation} message
 * @param {!jspb.BinaryWriter} writer
 */
proto.TextAnnotation.serializeBinaryToWriter = function(message, writer) {
  message.serializeBinaryToWriter(writer);
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.TextAnnotation.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  this.serializeBinaryToWriter(writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the message to binary data (in protobuf wire format),
 * writing to the given BinaryWriter.
 * @param {!jspb.BinaryWriter} writer
 */
proto.TextAnnotation.prototype.serializeBinaryToWriter = function (writer) {
  var f = undefined;
  f = this.getId();
  if (f !== 0) {
    writer.writeInt64(
      1,
      f
    );
  }
  f = this.getText();
  if (f.length > 0) {
    writer.writeString(
      2,
      f
    );
  }
};


/**
 * optional int64 id = 1;
 * @return {number}
 */
proto.TextAnnotation.prototype.getId = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 1, 0));
};


/** @param {number} value */
proto.TextAnnotation.prototype.setId = function(value) {
  jspb.Message.setField(this, 1, value);
};


/**
 * optional string text = 2;
 * @return {string}
 */
proto.TextAnnotation.prototype.getText = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/** @param {string} value */
proto.TextAnnotation.prototype.setText = function(value) {
  jspb.Message.setField(this, 2, value);
};



/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.StreetEasyAnnotation = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, proto.StreetEasyAnnotation.repeatedFields_, null);
};
goog.inherits(proto.StreetEasyAnnotation, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  proto.StreetEasyAnnotation.displayName = 'proto.StreetEasyAnnotation';
}
/**
 * List of repeated fields within this message type.
 * @private {!Array<number>}
 * @const
 */
proto.StreetEasyAnnotation.repeatedFields_ = [4];



if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto suitable for use in Soy templates.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     com.google.apps.jspb.JsClassTemplate.JS_RESERVED_WORDS.
 * @param {boolean=} opt_includeInstance Whether to include the JSPB instance
 *     for transitional soy proto support: http://goto/soy-param-migration
 * @return {!Object}
 */
proto.StreetEasyAnnotation.prototype.toObject = function(opt_includeInstance) {
  return proto.StreetEasyAnnotation.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Whether to include the JSPB
 *     instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.StreetEasyAnnotation} msg The msg instance to transform.
 * @return {!Object}
 */
proto.StreetEasyAnnotation.toObject = function(includeInstance, msg) {
  var f, obj = {
    hash: jspb.Message.getFieldWithDefault(msg, 1, 0),
    name: jspb.Message.getFieldWithDefault(msg, 2, ""),
    priceUsd: jspb.Message.getFieldWithDefault(msg, 3, 0),
    openHousesList: jspb.Message.getField(msg, 4)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.StreetEasyAnnotation}
 */
proto.StreetEasyAnnotation.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.StreetEasyAnnotation;
  return proto.StreetEasyAnnotation.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.StreetEasyAnnotation} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.StreetEasyAnnotation}
 */
proto.StreetEasyAnnotation.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {number} */ (reader.readUint64());
      msg.setHash(value);
      break;
    case 2:
      var value = /** @type {string} */ (reader.readString());
      msg.setName(value);
      break;
    case 3:
      var value = /** @type {number} */ (reader.readInt32());
      msg.setPriceUsd(value);
      break;
    case 4:
      var value = /** @type {string} */ (reader.readString());
      msg.addOpenHouses(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Class method variant: serializes the given message to binary data
 * (in protobuf wire format), writing to the given BinaryWriter.
 * @param {!proto.StreetEasyAnnotation} message
 * @param {!jspb.BinaryWriter} writer
 */
proto.StreetEasyAnnotation.serializeBinaryToWriter = function(message, writer) {
  message.serializeBinaryToWriter(writer);
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.StreetEasyAnnotation.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  this.serializeBinaryToWriter(writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the message to binary data (in protobuf wire format),
 * writing to the given BinaryWriter.
 * @param {!jspb.BinaryWriter} writer
 */
proto.StreetEasyAnnotation.prototype.serializeBinaryToWriter = function (writer) {
  var f = undefined;
  f = this.getHash();
  if (f !== 0) {
    writer.writeUint64(
      1,
      f
    );
  }
  f = this.getName();
  if (f.length > 0) {
    writer.writeString(
      2,
      f
    );
  }
  f = this.getPriceUsd();
  if (f !== 0) {
    writer.writeInt32(
      3,
      f
    );
  }
  f = this.getOpenHousesList();
  if (f.length > 0) {
    writer.writeRepeatedString(
      4,
      f
    );
  }
};


/**
 * optional uint64 hash = 1;
 * @return {number}
 */
proto.StreetEasyAnnotation.prototype.getHash = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 1, 0));
};


/** @param {number} value */
proto.StreetEasyAnnotation.prototype.setHash = function(value) {
  jspb.Message.setField(this, 1, value);
};


/**
 * optional string name = 2;
 * @return {string}
 */
proto.StreetEasyAnnotation.prototype.getName = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/** @param {string} value */
proto.StreetEasyAnnotation.prototype.setName = function(value) {
  jspb.Message.setField(this, 2, value);
};


/**
 * optional int32 price_usd = 3;
 * @return {number}
 */
proto.StreetEasyAnnotation.prototype.getPriceUsd = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 3, 0));
};


/** @param {number} value */
proto.StreetEasyAnnotation.prototype.setPriceUsd = function(value) {
  jspb.Message.setField(this, 3, value);
};


/**
 * repeated string open_houses = 4;
 * If you change this array by adding, removing or replacing elements, or if you
 * replace the array itself, then you must call the setter to update it.
 * @return {!Array.<string>}
 */
proto.StreetEasyAnnotation.prototype.getOpenHousesList = function() {
  return /** @type {!Array.<string>} */ (jspb.Message.getField(this, 4));
};


/** @param {!Array.<string>} value */
proto.StreetEasyAnnotation.prototype.setOpenHousesList = function(value) {
  jspb.Message.setField(this, 4, value || []);
};


/**
 * @param {!string} value
 * @param {number=} opt_index
 */
proto.StreetEasyAnnotation.prototype.addOpenHouses = function(value, opt_index) {
  jspb.Message.addToRepeatedField(this, 4, value, opt_index);
};


proto.StreetEasyAnnotation.prototype.clearOpenHousesList = function() {
  this.setOpenHousesList([]);
};


goog.object.extend(exports, proto);
