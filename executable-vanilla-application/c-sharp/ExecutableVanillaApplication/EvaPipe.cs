using ApplicationManagement.Local;
using ExecutableVanillaApplication.Utils;
using Google.Protobuf;
using System;
using System.Buffers;
using System.IO;
using System.IO.Pipes;

namespace ExecutableVanillaApplication
{
	internal class EvaPipe
	{
		private readonly NamedPipeClientStream _pipe;
		private readonly CircularBuffer _readBuffer;
		private readonly CircularBuffer _writeBuffer;

		private VarInt _nextMessageLength;

		public bool IsConnected => _pipe.IsConnected;

		public EvaPipe(string pipeName)
		{
			_pipe = new NamedPipeClientStream(pipeName);
			_readBuffer = new CircularBuffer();
			_writeBuffer = new CircularBuffer();

			_pipe.Connect();
		}

		private void TryConnecting()
		{

		}

		public void Read()
		{
			if (_readBuffer.IsFull)
				_readBuffer.Expand();

			if (!_readBuffer.IsWrapped)
				ReadIntoBuffer();

			if (_readBuffer.IsWrapped)
				ReadIntoBuffer();
		}

		private void ReadIntoBuffer()
		{
			_readBuffer.Accept(_pipe.Read(_readBuffer.NextAvailableChunk));

			var memoryStream = new MemoryStream();
			_pipe.Read()
				_pi
		}

		public bool TryParse(out ManagerToApplicationMessage message)
		{
			message = default;

			var sequence = _readBuffer.AsReadOnlySequence();
			var sequenceLength = sequence.Length;

			while (!_nextMessageLength.IsComplete && sequence.Length > 0)
			{
				_nextMessageLength = _nextMessageLength.With(sequence.First.Span[0]);

				sequence = sequence.Slice(1);
			}

			if (_nextMessageLength.IsComplete && _nextMessageLength.Value <= sequence.Length)
			{
				message = ManagerToApplicationMessage.Parser.ParseFrom(sequence);
				sequence = sequence.Slice(_nextMessageLength.Value);

				_nextMessageLength = default;
			}

			_readBuffer.Advance((int)(sequenceLength - sequence.Length));

			return message != default;
		}

		public void Encode(ApplicationToManagerMessage message)
		{
			message.WriteDelimitedTo(_pipe);
		}
	}
}
