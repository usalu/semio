Feature: GLB JSON receiving yields under original controls through typed glTF projection
 Scenario: Decode and project a private JSON chunk
  Given an original captured finite GLB byte source
  When JSON UTF8 decoding, lossless syntax and schema projection run
  Then each phase yields at original bounded interior work and completion
  And cancellation cannot return a partial artifact
  And later caller mutation cannot change the private source
 Scenario: Share the glTF declaration member tables
  Given duplicate members, canonical integer object keys and exact binary64 literals
  When the lossless syntax is projected with the existing glTF wire roles
  Then last decoded members win and object order matches ECMAScript enumeration
  And finite number words, safe indices, defaults, closed records and open extras agree
  And unpaired escaped surrogates and a single leading UTF8 BOM retain existing behavior
