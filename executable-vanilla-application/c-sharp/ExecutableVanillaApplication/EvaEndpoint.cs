using ExecutableVanillaApplication.Utils;
using Google.Protobuf;
using System;
using System.Buffers;
using System.IO.Pipes;
using WorkstationManagement.LocalManagement;

namespace ExecutableVanillaApplication
{
	public class EvaEndpoint
	{
		private readonly NamedPipeClientStream _pipe;
		private readonly CircularBuffer _buffer;

		private VarInt _nextMessageLength;

		public EvaEndpoint(string pipeName)
		{
			_pipe = new NamedPipeClientStream(pipeName);
			_buffer = new CircularBuffer();

			_pipe.Connect();
		}

		public void Read()
		{
			if (_buffer.IsFull)
				_buffer.Expand();

			if (!_buffer.IsWrapped)
				ReadIntoBuffer();

			if (_buffer.IsWrapped)
				ReadIntoBuffer();
		}

		private void ReadIntoBuffer()
		{
			_buffer.Accept(_pipe.Read(_buffer.NextAvailableChunk));
		}

		private bool TryParse(out Request request)
		{
			request = default;

			var sequence = _buffer.AsReadOnlySequence();
			var sequenceLength = sequence.Length;

			while (!_nextMessageLength.IsComplete && sequence.Length > 0)
			{
				_nextMessageLength = _nextMessageLength.With(sequence.First.Span[0]);

				sequence = sequence.Slice(1);
			}

			if (_nextMessageLength.IsComplete && _nextMessageLength.Value <= sequence.Length)
			{
				request = Request.Parser.ParseFrom(sequence);
				sequence = sequence.Slice(_nextMessageLength.Value);

				_nextMessageLength = default;
			}

			_buffer.Advance((int)(sequenceLength - sequence.Length));

			return request != default;
		}
	}
}
