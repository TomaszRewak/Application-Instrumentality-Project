using System;
using System.Buffers;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace ExecutableVanillaApplication.Utils
{
	internal sealed class CircularBuffer
	{
		byte[] _data = new byte[1024];
		int _dataBegin;
		int _dataLength;

		public bool IsFull => _dataLength == _data.Length;
		public bool IsWrapped => _dataBegin + _dataLength >= _data.Length;

		public Span<byte> NextAvailableChunk => IsWrapped
			? _data.AsSpan(_dataBegin + _dataLength - _data.Length, _data.Length - _dataLength)
			: _data.AsSpan(_dataBegin, _data.Length - _dataBegin - _dataLength);

		public ReadOnlySequence<byte> AsReadOnlySequence()
		{
			var firstDataChunkSize = Math.Min(_dataLength, _data.Length - _dataBegin);
			var secondDataChunkSize = _dataLength - firstDataChunkSize;

			if (secondDataChunkSize == 0)
			{
				return new ReadOnlySequence<byte>(_data.AsMemory(_dataBegin, _dataLength));
			}
			else
			{
				var firstMemorySegment = new MemorySegment<byte>(_data.AsMemory(_dataBegin, firstDataChunkSize));
				var secondMemorySegment = firstMemorySegment.Append(_data.AsMemory(0, secondDataChunkSize));

				return new ReadOnlySequence<byte>(firstMemorySegment, 0, secondMemorySegment, secondDataChunkSize);
			}
		}

		public void Accept(int length)
		{
			if (length > NextAvailableChunk.Length)
				throw new ArgumentOutOfRangeException(nameof(length));

			_dataLength += length;
		}

		public void Advance(int length)
		{
			if (length > _dataLength)
				throw new ArgumentOutOfRangeException(nameof(length));

			_dataBegin = (_dataBegin + length) % _data.Length;
			_dataLength -= length;
		}

		public void Expand()
		{
			var newData = new byte[_data.Length * 2];

			var firstDataChunkSize = Math.Min(_dataLength, _data.Length - _dataBegin);
			var secondDataChunkSize = _dataLength - firstDataChunkSize;

			Array.Copy(_data, _dataBegin, newData, 0, firstDataChunkSize);

			if (secondDataChunkSize > 0)
			{
				Array.Copy(_data, 0, newData, firstDataChunkSize, secondDataChunkSize);
			}

			_data = newData;
			_dataBegin = 0;
		}
	}
}
