/*
Copyright (C) 2018-2019 de4dot@gmail.com

Permission is hereby granted, free of charge, to any person obtaining
a copy of this software and associated documentation files (the
"Software"), to deal in the Software without restriction, including
without limitation the rights to use, copy, modify, merge, publish,
distribute, sublicense, and/or sell copies of the Software, and to
permit persons to whom the Software is furnished to do so, subject to
the following conditions:

The above copyright notice and this permission notice shall be
included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/

#if !NO_INTEL_FORMATTER && !NO_FORMATTER
using System;
using Iced.Intel.FormatterInternal;

namespace Iced.Intel.IntelFormatterInternal {
	static class MemorySizes {
		public readonly struct Info {
			public readonly FormatterString[] keywords;
			public readonly FormatterString bcstTo;
			public Info(FormatterString[] keywords, FormatterString bcstTo) {
				this.keywords = keywords;
				this.bcstTo = bcstTo;
			}
		}
		public static readonly Info[] AllMemorySizes = GetMemorySizes();
		enum MemoryKeywords {
			None,
			byte_ptr,
			dword_ptr,
			fpuenv14_ptr,
			fpuenv28_ptr,
			fpustate108_ptr,
			fpustate94_ptr,
			fword_ptr,
			qword_ptr,
			tbyte_ptr,
			word_ptr,
			xmmword_ptr,
			ymmword_ptr,
			zmmword_ptr,
		}
		enum BroadcastToKind {
			None,
			b1to2,
			b1to4,
			b1to8,
			b1to16,
		}
		static Info[] GetMemorySizes() {
			var ptr = new FormatterString("ptr");
			var byte_ptr = new[] { new FormatterString("byte"), ptr };
			var word_ptr = new[] { new FormatterString("word"), ptr };
			var dword_ptr = new[] { new FormatterString("dword"), ptr };
			var qword_ptr = new[] { new FormatterString("qword"), ptr };
			var xmmword_ptr = new[] { new FormatterString("xmmword"), ptr };
			var ymmword_ptr = new[] { new FormatterString("ymmword"), ptr };
			var zmmword_ptr = new[] { new FormatterString("zmmword"), ptr };
			var fword_ptr = new[] { new FormatterString("fword"), ptr };
			var tbyte_ptr = new[] { new FormatterString("tbyte"), ptr };
			var fpuenv14_ptr = new[] { new FormatterString("fpuenv14"), ptr };
			var fpuenv28_ptr = new[] { new FormatterString("fpuenv28"), ptr };
			var fpustate108_ptr = new[] { new FormatterString("fpustate108"), ptr };
			var fpustate94_ptr = new[] { new FormatterString("fpustate94"), ptr };

			var infos = new Info[IcedConstants.NumberOfMemorySizes];
			const int BroadcastToKindShift = 5;
			const int MemoryKeywordsMask = 0x1F;
#if HAS_SPAN
			ReadOnlySpan<byte>
#else
			byte[]
#endif
			data = new byte[IcedConstants.NumberOfMemorySizes] {
				// GENERATOR-BEGIN: MemorySizes
				// ⚠️This was generated by GENERATOR!🦹‍♂️
				(byte)((uint)MemoryKeywords.None | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.byte_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.word_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.dword_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.qword_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.qword_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.xmmword_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.ymmword_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.zmmword_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.byte_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.word_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.dword_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.qword_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.xmmword_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.ymmword_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.zmmword_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.dword_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.fword_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.tbyte_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.word_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.dword_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.qword_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.dword_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.qword_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.qword_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.xmmword_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.fword_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.fword_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.word_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.dword_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.qword_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.tbyte_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.xmmword_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.word_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.fpuenv14_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.fpuenv28_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.fpustate94_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.fpustate108_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.None | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.None | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.None | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.None | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.tbyte_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.word_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.word_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.dword_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.dword_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.dword_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.dword_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.dword_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.qword_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.qword_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.qword_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.qword_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.qword_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.qword_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.qword_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.qword_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.xmmword_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.xmmword_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.xmmword_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.xmmword_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.xmmword_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.xmmword_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.xmmword_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.xmmword_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.xmmword_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.xmmword_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.xmmword_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.xmmword_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.xmmword_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.ymmword_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.ymmword_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.ymmword_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.ymmword_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.ymmword_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.ymmword_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.ymmword_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.ymmword_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.ymmword_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.ymmword_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.ymmword_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.ymmword_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.ymmword_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.ymmword_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.ymmword_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.ymmword_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.zmmword_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.zmmword_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.zmmword_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.zmmword_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.zmmword_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.zmmword_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.zmmword_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.zmmword_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.zmmword_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.zmmword_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.zmmword_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.zmmword_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.zmmword_ptr | ((uint)BroadcastToKind.None << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.dword_ptr | ((uint)BroadcastToKind.b1to2 << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.dword_ptr | ((uint)BroadcastToKind.b1to2 << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.dword_ptr | ((uint)BroadcastToKind.b1to2 << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.dword_ptr | ((uint)BroadcastToKind.b1to4 << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.dword_ptr | ((uint)BroadcastToKind.b1to4 << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.qword_ptr | ((uint)BroadcastToKind.b1to2 << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.qword_ptr | ((uint)BroadcastToKind.b1to2 << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.qword_ptr | ((uint)BroadcastToKind.b1to2 << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.dword_ptr | ((uint)BroadcastToKind.b1to4 << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.qword_ptr | ((uint)BroadcastToKind.b1to2 << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.dword_ptr | ((uint)BroadcastToKind.b1to8 << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.dword_ptr | ((uint)BroadcastToKind.b1to8 << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.qword_ptr | ((uint)BroadcastToKind.b1to4 << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.qword_ptr | ((uint)BroadcastToKind.b1to4 << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.qword_ptr | ((uint)BroadcastToKind.b1to4 << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.dword_ptr | ((uint)BroadcastToKind.b1to8 << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.qword_ptr | ((uint)BroadcastToKind.b1to4 << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.dword_ptr | ((uint)BroadcastToKind.b1to16 << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.dword_ptr | ((uint)BroadcastToKind.b1to16 << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.qword_ptr | ((uint)BroadcastToKind.b1to8 << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.qword_ptr | ((uint)BroadcastToKind.b1to8 << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.qword_ptr | ((uint)BroadcastToKind.b1to8 << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.dword_ptr | ((uint)BroadcastToKind.b1to16 << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.qword_ptr | ((uint)BroadcastToKind.b1to8 << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.dword_ptr | ((uint)BroadcastToKind.b1to4 << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.dword_ptr | ((uint)BroadcastToKind.b1to8 << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.dword_ptr | ((uint)BroadcastToKind.b1to16 << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.qword_ptr | ((uint)BroadcastToKind.b1to2 << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.qword_ptr | ((uint)BroadcastToKind.b1to4 << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.qword_ptr | ((uint)BroadcastToKind.b1to8 << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.qword_ptr | ((uint)BroadcastToKind.b1to2 << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.qword_ptr | ((uint)BroadcastToKind.b1to4 << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.qword_ptr | ((uint)BroadcastToKind.b1to8 << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.dword_ptr | ((uint)BroadcastToKind.b1to4 << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.dword_ptr | ((uint)BroadcastToKind.b1to8 << BroadcastToKindShift)),
				(byte)((uint)MemoryKeywords.dword_ptr | ((uint)BroadcastToKind.b1to16 << BroadcastToKindShift)),
				// GENERATOR-END: MemorySizes
			};

			var b1to2 = new FormatterString("1to2");
			var b1to4 = new FormatterString("1to4");
			var b1to8 = new FormatterString("1to8");
			var b1to16 = new FormatterString("1to16");
			for (int i = 0; i < infos.Length; i++) {
				var d = data[i];
				var keywords = ((MemoryKeywords)(d & MemoryKeywordsMask)) switch {
					MemoryKeywords.None => Array2.Empty<FormatterString>(),
					MemoryKeywords.byte_ptr => byte_ptr,
					MemoryKeywords.dword_ptr => dword_ptr,
					MemoryKeywords.fpuenv14_ptr => fpuenv14_ptr,
					MemoryKeywords.fpuenv28_ptr => fpuenv28_ptr,
					MemoryKeywords.fpustate108_ptr => fpustate108_ptr,
					MemoryKeywords.fpustate94_ptr => fpustate94_ptr,
					MemoryKeywords.fword_ptr => fword_ptr,
					MemoryKeywords.qword_ptr => qword_ptr,
					MemoryKeywords.tbyte_ptr => tbyte_ptr,
					MemoryKeywords.word_ptr => word_ptr,
					MemoryKeywords.xmmword_ptr => xmmword_ptr,
					MemoryKeywords.ymmword_ptr => ymmword_ptr,
					MemoryKeywords.zmmword_ptr => zmmword_ptr,
					_ => throw new InvalidOperationException(),
				};
				var bcstTo = ((BroadcastToKind)(d >> BroadcastToKindShift)) switch {
					BroadcastToKind.None => default,
					BroadcastToKind.b1to2 => b1to2,
					BroadcastToKind.b1to4 => b1to4,
					BroadcastToKind.b1to8 => b1to8,
					BroadcastToKind.b1to16 => b1to16,
					_ => throw new InvalidOperationException(),
				};
				infos[i] = new Info(keywords, bcstTo);
			}

			return infos;
		}
	}
}
#endif
