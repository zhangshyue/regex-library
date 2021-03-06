// source: root.proto
/**
 * @fileoverview
 * @enhanceable
 * @suppress {messageConventions} JS Compiler reports an error if a variable or
 *     field starts with 'MSG_' and isn't a translatable message.
 * @public
 */
// GENERATED CODE -- DO NOT EDIT!
/* eslint-disable */
// @ts-nocheck

goog.provide('proto.FileOutput');

goog.require('jspb.BinaryReader');
goog.require('jspb.BinaryWriter');
goog.require('jspb.Message');
goog.require('proto.Output');
goog.require('proto.Root');

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
proto.FileOutput = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, proto.FileOutput.repeatedFields_, null);
};
goog.inherits(proto.FileOutput, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.FileOutput.displayName = 'proto.FileOutput';
}

/**
 * List of repeated fields within this message type.
 * @private {!Array<number>}
 * @const
 */
proto.FileOutput.repeatedFields_ = [2];



if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.FileOutput.prototype.toObject = function(opt_includeInstance) {
  return proto.FileOutput.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.FileOutput} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.FileOutput.toObject = function(includeInstance, msg) {
  var f, obj = {
    root: (f = msg.getRoot()) && proto.Root.toObject(includeInstance, f),
    outputsList: jspb.Message.toObjectList(msg.getOutputsList(),
    proto.Output.toObject, includeInstance)
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
 * @return {!proto.FileOutput}
 */
proto.FileOutput.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.FileOutput;
  return proto.FileOutput.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.FileOutput} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.FileOutput}
 */
proto.FileOutput.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = new proto.Root;
      reader.readMessage(value,proto.Root.deserializeBinaryFromReader);
      msg.setRoot(value);
      break;
    case 2:
      var value = new proto.Output;
      reader.readMessage(value,proto.Output.deserializeBinaryFromReader);
      msg.addOutputs(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.FileOutput.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.FileOutput.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.FileOutput} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.FileOutput.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getRoot();
  if (f != null) {
    writer.writeMessage(
      1,
      f,
      proto.Root.serializeBinaryToWriter
    );
  }
  f = message.getOutputsList();
  if (f.length > 0) {
    writer.writeRepeatedMessage(
      2,
      f,
      proto.Output.serializeBinaryToWriter
    );
  }
};


/**
 * optional Root root = 1;
 * @return {?proto.Root}
 */
proto.FileOutput.prototype.getRoot = function() {
  return /** @type{?proto.Root} */ (
    jspb.Message.getWrapperField(this, proto.Root, 1));
};


/**
 * @param {?proto.Root|undefined} value
 * @return {!proto.FileOutput} returns this
*/
proto.FileOutput.prototype.setRoot = function(value) {
  return jspb.Message.setWrapperField(this, 1, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.FileOutput} returns this
 */
proto.FileOutput.prototype.clearRoot = function() {
  return this.setRoot(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.FileOutput.prototype.hasRoot = function() {
  return jspb.Message.getField(this, 1) != null;
};


/**
 * repeated Output outputs = 2;
 * @return {!Array<!proto.Output>}
 */
proto.FileOutput.prototype.getOutputsList = function() {
  return /** @type{!Array<!proto.Output>} */ (
    jspb.Message.getRepeatedWrapperField(this, proto.Output, 2));
};


/**
 * @param {!Array<!proto.Output>} value
 * @return {!proto.FileOutput} returns this
*/
proto.FileOutput.prototype.setOutputsList = function(value) {
  return jspb.Message.setRepeatedWrapperField(this, 2, value);
};


/**
 * @param {!proto.Output=} opt_value
 * @param {number=} opt_index
 * @return {!proto.Output}
 */
proto.FileOutput.prototype.addOutputs = function(opt_value, opt_index) {
  return jspb.Message.addToRepeatedWrapperField(this, 2, opt_value, proto.Output, opt_index);
};


/**
 * Clears the list making it empty but non-null.
 * @return {!proto.FileOutput} returns this
 */
proto.FileOutput.prototype.clearOutputsList = function() {
  return this.setOutputsList([]);
};


