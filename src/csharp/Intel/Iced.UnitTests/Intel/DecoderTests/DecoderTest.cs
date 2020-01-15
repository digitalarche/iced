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

using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Runtime.CompilerServices;
using Iced.Intel;
using Xunit;

namespace Iced.UnitTests.Intel.DecoderTests {
	public abstract class DecoderTest {
		protected Decoder CreateDecoder16(string hexBytes, [CallerMemberName] string callerName = null) =>
			CreateDecoder(16, callerName, hexBytes, DecoderOptions.None);
		protected Decoder CreateDecoder32(string hexBytes, [CallerMemberName] string callerName = null) =>
			CreateDecoder(32, callerName, hexBytes, DecoderOptions.None);
		protected Decoder CreateDecoder64(string hexBytes, [CallerMemberName] string callerName = null) =>
			CreateDecoder(64, callerName, hexBytes, DecoderOptions.None);

		Decoder CreateDecoder(int bitness, string callerName, string hexBytes, DecoderOptions options) {
			Assert.StartsWith("Test" + bitness.ToString(), callerName);
			return CreateDecoder(bitness, hexBytes, options).decoder;
		}

		(Decoder decoder, int length, bool canRead, ByteArrayCodeReader codeReader) CreateDecoder(int codeSize, string hexBytes, DecoderOptions options) {
			var codeReader = new ByteArrayCodeReader(hexBytes);
			var decoder = Decoder.Create(codeSize, codeReader, options);
			decoder.IP = codeSize switch {
				16 => DecoderConstants.DEFAULT_IP16,
				32 => DecoderConstants.DEFAULT_IP32,
				64 => DecoderConstants.DEFAULT_IP64,
				_ => throw new ArgumentOutOfRangeException(nameof(codeSize)),
			};
			Assert.Equal(codeSize, decoder.Bitness);
			int length = Math.Min(IcedConstants.MaxInstructionLength, codeReader.Count);
			bool canRead = length < codeReader.Count;
			return (decoder, length, canRead, codeReader);
		}

		protected void DecodeMemOpsBase(int bitness, string hexBytes, Code code, Register register, Register prefixSeg, Register segReg, Register baseReg, Register indexReg, int scale, uint displ, int displSize, in ConstantOffsets constantOffsets, string encodedHexBytes, DecoderOptions options) {
			var (decoder, length, canRead, codeReader) = CreateDecoder(bitness, hexBytes, options);
			var instr = decoder.Decode();
			Assert.Equal(canRead, codeReader.CanReadByte);

			Assert.Equal(code, instr.Code);
			Assert.Equal(2, instr.OpCount);
			Assert.Equal(length, instr.Length);
			Assert.False(instr.HasRepPrefix);
			Assert.False(instr.HasRepePrefix);
			Assert.False(instr.HasRepnePrefix);
			Assert.False(instr.HasLockPrefix);
			Assert.Equal(prefixSeg, instr.SegmentPrefix);
			if (instr.SegmentPrefix == Register.None)
				Assert.False(instr.HasSegmentPrefix);
			else
				Assert.True(instr.HasSegmentPrefix);

			Assert.Equal(OpKind.Memory, instr.Op0Kind);
			Assert.Equal(segReg, instr.MemorySegment);
			Assert.Equal(baseReg, instr.MemoryBase);
			Assert.Equal(indexReg, instr.MemoryIndex);
			Assert.Equal(displ, instr.MemoryDisplacement);
			Assert.Equal((ulong)(int)displ, instr.MemoryDisplacement64);
			Assert.Equal(1 << scale, instr.MemoryIndexScale);
			Assert.Equal(displSize, instr.MemoryDisplSize);

			Assert.Equal(OpKind.Register, instr.Op1Kind);
			Assert.Equal(register, instr.Op1Register);
			VerifyConstantOffsets(constantOffsets, decoder.GetConstantOffsets(instr));
		}

		protected static IEnumerable<object[]> GetMemOpsData(int bitness) {
			var allTestCases = DecoderTestCases.GetMemoryTestCases(bitness);
			foreach (var tc in allTestCases)
				yield return new object[13] { tc.HexBytes, tc.Code, tc.Register, tc.SegmentPrefix, tc.SegmentRegister, tc.BaseRegister, tc.IndexRegister, tc.Scale, tc.Displacement, tc.DisplacementSize, tc.ConstantOffsets, tc.EncodedHexBytes, tc.DecoderOptions };
		}

		protected static IEnumerable<object[]> GetDecoderTestData(int bitness, int classIndex) {
			var allTestCases = DecoderTestCases.GetTestCases(bitness);
			const int TotalClasses = 8;
			if ((uint)classIndex >= (uint)TotalClasses)
				throw new InvalidOperationException();
			int countPerClass = (allTestCases.Length + TotalClasses - 1) / TotalClasses;
			int startIndex = classIndex * countPerClass;
			int endIndex = Math.Min(allTestCases.Length, startIndex + countPerClass);
			object boxedBitness = bitness;
			while (startIndex < endIndex) {
				var tc = allTestCases[startIndex++];
				Debug.Assert(bitness == tc.Bitness);
				yield return new object[4] { boxedBitness, tc.LineNumber, tc.HexBytes, tc };
			}
		}

		protected static IEnumerable<object[]> GetMiscDecoderTestData(int bitness) {
			var allTestCases = DecoderTestCases.GetMiscTestCases(bitness);
			object boxedBitness = bitness;
			foreach (var tc in allTestCases) {
				Debug.Assert(bitness == tc.Bitness);
				yield return new object[4] { boxedBitness, tc.LineNumber, tc.HexBytes, tc };
			}
		}

		internal void DecoderTestBase(int bitness, int lineNo, string hexBytes, DecoderTestCase tc) {
			var (decoder, length, canRead, codeReader) = CreateDecoder(bitness, hexBytes, tc.DecoderOptions);
			ulong rip = decoder.IP;
			decoder.Decode(out var instr);
			Assert.Equal(canRead, codeReader.CanReadByte);
			Assert.Equal(tc.Code, instr.Code);
			Assert.Equal(tc.Mnemonic, instr.Mnemonic);
			Assert.Equal(instr.Mnemonic, instr.Code.Mnemonic());
			Assert.Equal(length, instr.Length);
			Assert.Equal(rip, instr.IP);
			Assert.Equal(decoder.IP, instr.NextIP);
			Assert.Equal(tc.OpCount, instr.OpCount);
			Assert.Equal(tc.ZeroingMasking, instr.ZeroingMasking);
			Assert.Equal(!tc.ZeroingMasking, instr.MergingMasking);
			Assert.Equal(tc.SuppressAllExceptions, instr.SuppressAllExceptions);
			Assert.Equal(tc.IsBroadcast, instr.IsBroadcast);
			Assert.Equal(tc.HasXacquirePrefix, instr.HasXacquirePrefix);
			Assert.Equal(tc.HasXreleasePrefix, instr.HasXreleasePrefix);
			Assert.Equal(tc.HasRepePrefix, instr.HasRepPrefix);
			Assert.Equal(tc.HasRepePrefix, instr.HasRepePrefix);
			Assert.Equal(tc.HasRepnePrefix, instr.HasRepnePrefix);
			Assert.Equal(tc.HasLockPrefix, instr.HasLockPrefix);
			switch (tc.VsibBitness) {
			case 0:
				Assert.False(instr.IsVsib);
				Assert.False(instr.IsVsib32);
				Assert.False(instr.IsVsib64);
				Assert.False(instr.TryGetVsib64(out _));
				break;

			case 32:
				Assert.True(instr.IsVsib);
				Assert.True(instr.IsVsib32);
				Assert.False(instr.IsVsib64);
				Assert.True(instr.TryGetVsib64(out bool vsib64) && !vsib64);
				break;

			case 64:
				Assert.True(instr.IsVsib);
				Assert.False(instr.IsVsib32);
				Assert.True(instr.IsVsib64);
				Assert.True(instr.TryGetVsib64(out vsib64) && vsib64);
				break;

			default:
				throw new InvalidOperationException();
			}
			Assert.Equal(tc.OpMask, instr.OpMask);
			Assert.Equal(tc.OpMask != Register.None, instr.HasOpMask);
			Assert.Equal(tc.RoundingControl, instr.RoundingControl);
			Assert.Equal(tc.SegmentPrefix, instr.SegmentPrefix);
			if (instr.SegmentPrefix == Register.None)
				Assert.False(instr.HasSegmentPrefix);
			else
				Assert.True(instr.HasSegmentPrefix);
			for (int i = 0; i < tc.OpCount; i++) {
				var opKind = tc.GetOpKind(i);
				Assert.Equal(opKind, instr.GetOpKind(i));
				switch (opKind) {
				case OpKind.Register:
					Assert.Equal(tc.GetOpRegister(i), instr.GetOpRegister(i));
					break;

				case OpKind.NearBranch16:
					Assert.Equal(tc.NearBranch, instr.NearBranch16);
					break;

				case OpKind.NearBranch32:
					Assert.Equal(tc.NearBranch, instr.NearBranch32);
					break;

				case OpKind.NearBranch64:
					Assert.Equal(tc.NearBranch, instr.NearBranch64);
					break;

				case OpKind.FarBranch16:
					Assert.Equal(tc.FarBranch, instr.FarBranch16);
					Assert.Equal(tc.FarBranchSelector, instr.FarBranchSelector);
					break;

				case OpKind.FarBranch32:
					Assert.Equal(tc.FarBranch, instr.FarBranch32);
					Assert.Equal(tc.FarBranchSelector, instr.FarBranchSelector);
					break;

				case OpKind.Immediate8:
					Assert.Equal((byte)tc.Immediate, instr.Immediate8);
					break;

				case OpKind.Immediate8_2nd:
					Assert.Equal(tc.Immediate_2nd, instr.Immediate8_2nd);
					break;

				case OpKind.Immediate16:
					Assert.Equal((ushort)tc.Immediate, instr.Immediate16);
					break;

				case OpKind.Immediate32:
					Assert.Equal((uint)tc.Immediate, instr.Immediate32);
					break;

				case OpKind.Immediate64:
					Assert.Equal(tc.Immediate, instr.Immediate64);
					break;

				case OpKind.Immediate8to16:
					Assert.Equal((short)tc.Immediate, instr.Immediate8to16);
					break;

				case OpKind.Immediate8to32:
					Assert.Equal((int)tc.Immediate, instr.Immediate8to32);
					break;

				case OpKind.Immediate8to64:
					Assert.Equal((long)tc.Immediate, instr.Immediate8to64);
					break;

				case OpKind.Immediate32to64:
					Assert.Equal((long)tc.Immediate, instr.Immediate32to64);
					break;

				case OpKind.MemorySegSI:
				case OpKind.MemorySegESI:
				case OpKind.MemorySegRSI:
				case OpKind.MemorySegDI:
				case OpKind.MemorySegEDI:
				case OpKind.MemorySegRDI:
					Assert.Equal(tc.MemorySegment, instr.MemorySegment);
					Assert.Equal(tc.MemorySize, instr.MemorySize);
					break;

				case OpKind.MemoryESDI:
				case OpKind.MemoryESEDI:
				case OpKind.MemoryESRDI:
					Assert.Equal(tc.MemorySize, instr.MemorySize);
					break;

				case OpKind.Memory64:
					Assert.Equal(tc.MemorySegment, instr.MemorySegment);
					Assert.Equal(tc.MemoryAddress64, instr.MemoryAddress64);
					Assert.Equal(tc.MemorySize, instr.MemorySize);
					break;

				case OpKind.Memory:
					Assert.Equal(tc.MemorySegment, instr.MemorySegment);
					Assert.Equal(tc.MemoryBase, instr.MemoryBase);
					Assert.Equal(tc.MemoryIndex, instr.MemoryIndex);
					Assert.Equal(tc.MemoryIndexScale, instr.MemoryIndexScale);
					Assert.Equal(tc.MemoryDisplacement, instr.MemoryDisplacement);
					Assert.Equal((ulong)(int)tc.MemoryDisplacement, instr.MemoryDisplacement64);
					Assert.Equal(tc.MemoryDisplSize, instr.MemoryDisplSize);
					Assert.Equal(tc.MemorySize, instr.MemorySize);
					break;

				default:
					throw new InvalidOperationException();
				}
			}
			if (tc.OpCount >= 1) {
				Assert.Equal(tc.Op0Kind, instr.Op0Kind);
				if (tc.Op0Kind == OpKind.Register)
					Assert.Equal(tc.Op0Register, instr.Op0Register);
				if (tc.OpCount >= 2) {
					Assert.Equal(tc.Op1Kind, instr.Op1Kind);
					if (tc.Op1Kind == OpKind.Register)
						Assert.Equal(tc.Op1Register, instr.Op1Register);
					if (tc.OpCount >= 3) {
						Assert.Equal(tc.Op2Kind, instr.Op2Kind);
						if (tc.Op2Kind == OpKind.Register)
							Assert.Equal(tc.Op2Register, instr.Op2Register);
						if (tc.OpCount >= 4) {
							Assert.Equal(tc.Op3Kind, instr.Op3Kind);
							if (tc.Op3Kind == OpKind.Register)
								Assert.Equal(tc.Op3Register, instr.Op3Register);
							if (tc.OpCount >= 5) {
								Assert.Equal(tc.Op4Kind, instr.Op4Kind);
								if (tc.Op4Kind == OpKind.Register)
									Assert.Equal(tc.Op4Register, instr.Op4Register);
								Assert.Equal(5, tc.OpCount);
							}
						}
					}
				}
			}
			VerifyConstantOffsets(tc.ConstantOffsets, decoder.GetConstantOffsets(instr));
		}

		protected static void VerifyConstantOffsets(in ConstantOffsets expected, in ConstantOffsets actual) {
			Assert.Equal(expected.ImmediateOffset, actual.ImmediateOffset);
			Assert.Equal(expected.ImmediateSize, actual.ImmediateSize);
			Assert.Equal(expected.ImmediateOffset2, actual.ImmediateOffset2);
			Assert.Equal(expected.ImmediateSize2, actual.ImmediateSize2);
			Assert.Equal(expected.DisplacementOffset, actual.DisplacementOffset);
			Assert.Equal(expected.DisplacementSize, actual.DisplacementSize);
		}
	}
}
