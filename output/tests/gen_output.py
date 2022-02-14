"""
Helper file to generate a file output type
"""

from root_pb2 import *
import base64

in_root = "Cm8vVXNlcnMvd2lsbC9Qcm9qZWN0cy9yZWdleF9leHRyYWN0aW9uL3Byb2dyYW1zL2V4dHJhY3Rpb24vdGVzdHMvdGVzdF9yZWZlcmVuY2VfZmlsZXMvanMvdGVzdF9jb21wbGV4X2V4YW1wbGUuanMQARh0IiQqBFxccysyCUICXFxIB4ABAzIIQgFzSAVyAXMyB0IBK0gCYAA="

r = Root()
r.ParseFromString(base64.b64decode(in_root))

fo = FileOutput()

o = Output()

o.status = "This is a bad regex"
o.score = 1


annotation = Annotation()
annotation.note = f""
relevant_token = r.expression.tokens[0]
annotation.token.CopyFrom(relevant_token)

o.annotations.append(annotation)

fo.root.CopyFrom(r)
fo.outputs.append(o)

print(base64.b64encode(fo.SerializeToString()).decode("utf-8"))