using System;
using System.IO.Pipes;

namespace ExecutableVanillaApplication
{
	public class EvaEndpoint
	{
		public void Start(string pipeName)
		{
			var pipe = new NamedPipeClientStream(pipeName);
			pipe.Connect();
		}
	}
}
