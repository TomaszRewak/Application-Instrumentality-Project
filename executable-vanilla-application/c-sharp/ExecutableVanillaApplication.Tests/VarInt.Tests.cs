using ExecutableVanillaApplication.Utils;
using NUnit.Framework;
using System;

namespace ExecutableVanillaApplication.Tests
{
	public class Tests
	{
		[Test]
		public void NewVarInt_IsNotComplete()
		{
			VarInt varInt = new();

			Assert.IsFalse(varInt.IsComplete);
		}

		[Test]
		public void NewVarInt_HasZeroChunks()
		{
			VarInt varInt = new();

			Assert.AreEqual(0, varInt.NumberOfChunks);
		}

		[TestCase(new byte[] { 0b0000_0000 })]
		[TestCase(new byte[] { 0b0111_1111 })]
		[TestCase(new byte[] { 0b0010_1010 })]
		[TestCase(new byte[] { 0b1010_1010, 0b0010_1010 })]
		[TestCase(new byte[] { 0b1010_1010, 0b1010_1010, 0b0010_1010 })]
		public void CompleteVarInt_IsComplete(byte[] chunks)
		{
			VarInt varInt = new();

			foreach (var chunk in chunks)
				varInt = varInt.With(chunk);

			Assert.IsTrue(varInt.IsComplete);
		}

		[TestCase(new byte[] { 0b1000_0000 })]
		[TestCase(new byte[] { 0b1010_1010 })]
		[TestCase(new byte[] { 0b1111_1111 })]
		[TestCase(new byte[] { 0b1010_1010, 0b1010_1010 })]
		[TestCase(new byte[] { 0b1010_1010, 0b1010_1010, 0b1010_1010 })]
		public void IncompleteVarInt_IsIncomplete(byte[] chunks)
		{
			VarInt varInt = new();

			foreach (var chunk in chunks)
				varInt = varInt.With(chunk);

			Assert.IsFalse(varInt.IsComplete);
		}


		[TestCase(1, new byte[] { 0b0000_0000 })]
		[TestCase(1, new byte[] { 0b1111_1111 })]
		[TestCase(2, new byte[] { 0b1010_1010, 0b0010_1010 })]
		[TestCase(3, new byte[] { 0b1010_1010, 0b1010_1010, 0b1010_1010 })]
		public void Chunks_AreCounted(int expected, byte[] chunks)
		{
			VarInt varInt = new();

			foreach (var chunk in chunks)
				varInt = varInt.With(chunk);

			Assert.AreEqual(expected, varInt.NumberOfChunks);
		}

		[TestCase(1, new byte[] { 0b0000_0001 })]
		[TestCase(150, new byte[] { 0b1001_0110, 0b0000_0001 })]
		[TestCase(300, new byte[] { 0b1010_1100, 0b0000_0010 })]
		public void Bytes_AreDeserialized(int expected, byte[] chunks)
		{
			VarInt varInt = new();

			foreach (var chunk in chunks)
				varInt = varInt.With(chunk);

			Assert.AreEqual(expected, varInt.Value);
		}

		[TestCase(new byte[] { 0b0000_0001, 0b0000_0001 })]
		[TestCase(new byte[] { 0b1001_0110, 0b0000_0001, 0b0000_0001 })]
		public void VarInt_ThrowsOnExcessiveChunks(byte[] chunks)
		{
			VarInt varInt = new();

			Assert.Throws<InvalidOperationException>(() => {
				foreach (var chunk in chunks)
					varInt = varInt.With(chunk);
			});
		}
	}
}