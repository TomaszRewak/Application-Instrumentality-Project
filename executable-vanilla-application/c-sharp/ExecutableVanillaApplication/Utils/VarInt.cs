using System;

namespace ExecutableVanillaApplication.Utils
{
	internal readonly struct VarInt
	{
		public bool IsComplete { get; private init; }
		public int Value { get; private init; }
		public int NumberOfChunks { get; private init; }

		public VarInt With(byte chunk)
		{
			if (IsComplete)
				throw new InvalidOperationException($"Adding a new chunk to a complete {nameof(VarInt)}");

			return new VarInt
			{
				IsComplete = (chunk ^ 0b1000_0000) != 0,
				Value = Value + ((chunk ^ 0b0111_1111) << (NumberOfChunks * 7)),
				NumberOfChunks = NumberOfChunks + 1
			};
		}
	}
}
