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

goog.provide('proto.Root');

goog.require('jspb.BinaryReader');
goog.require('jspb.BinaryWriter');
goog.require('jspb.Message');
goog.require('proto.Expression');

goog.forwardDeclare('proto.SupportedLanguage');
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
proto.Root = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.Root, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.Root.displayName = 'proto.Root';
}



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
proto.Root.prototype.toObject = function(opt_includeInstance) {
  return proto.Root.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.Root} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.Root.toObject = function(includeInstance, msg) {
  var f, obj = {
    file: jspb.Message.getFieldWithDefault(msg, 1, ""),
    language: jspb.Message.getFieldWithDefault(msg, 2, 0),
    lineNumber: jspb.Message.getFieldWithDefault(msg, 3, 0),
    expression: (f = msg.getExpression()) && proto.Expression.toObject(includeInstance, f)
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
 * @return {!proto.Root}
 */
proto.Root.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.Root;
  return proto.Root.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.Root} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.Root}
 */
proto.Root.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setFile(value);
      break;
    case 2:
      var value = /** @type {!proto.SupportedLanguage} */ (reader.readEnum());
      msg.setLanguage(value);
      break;
    case 3:
      var value = /** @type {number} */ (reader.readInt32());
      msg.setLineNumber(value);
      break;
    case 4:
      var value = new proto.Expression;
      reader.readMessage(value,proto.Expression.deserializeBinaryFromReader);
      msg.setExpression(value);
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
proto.Root.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.Root.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.Root} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.Root.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getFile();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = message.getLanguage();
  if (f !== 0.0) {
    writer.writeEnum(
      2,
      f
    );
  }
  f = message.getLineNumber();
  if (f !== 0) {
    writer.writeInt32(
      3,
      f
    );
  }
  f = message.getExpression();
  if (f != null) {
    writer.writeMessage(
      4,
      f,
      proto.Expression.serializeBinaryToWriter
    );
  }
};


/**
 * optional string file = 1;
 * @return {string}
 */
proto.Root.prototype.getFile = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.Root} returns this
 */
proto.Root.prototype.setFile = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional SupportedLanguage language = 2;
 * @return {!proto.SupportedLanguage}
 */
proto.Root.prototype.getLanguage = function() {
  return /** @type {!proto.SupportedLanguage} */ (jspb.Message.getFieldWithDefault(this, 2, 0));
};


/**
 * @param {!proto.SupportedLanguage} value
 * @return {!proto.Root} returns this
 */
proto.Root.prototype.setLanguage = function(value) {
  return jspb.Message.setProto3EnumField(this, 2, value);
};


/**
 * optional int32 line_number = 3;
 * @return {number}
 */
proto.Root.prototype.getLineNumber = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 3, 0));
};


/**
 * @param {number} value
 * @return {!proto.Root} returns this
 */
proto.Root.prototype.setLineNumber = function(value) {
  return jspb.Message.setProto3IntField(this, 3, value);
};


/**
 * optional Expression expression = 4;
 * @return {?proto.Expression}
 */
proto.Root.prototype.getExpression = function() {
  return /** @type{?proto.Expression} */ (
    jspb.Message.getWrapperField(this, proto.Expression, 4));
};


/**
 * @param {?proto.Expression|undefined} value
 * @return {!proto.Root} returns this
*/
proto.Root.prototype.setExpression = function(value) {
  return jspb.Message.setWrapperField(this, 4, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.Root} returns this
 */
proto.Root.prototype.clearExpression = function() {
  return this.setExpression(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.Root.prototype.hasExpression = function() {
  return jspb.Message.getField(this, 4) != null;
};


