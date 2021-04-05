using System;
using System.Diagnostics;

namespace Magi.Execution
{
	public class ExecutableVanillaApplication
	{
		private readonly string _application;
		private readonly string _args;

		public void Start()
		{
			var process = Process.Start(new ProcessStartInfo(_application, _args));
		}

		public void SendSignal(string signal)
		{

		}
	}
}
