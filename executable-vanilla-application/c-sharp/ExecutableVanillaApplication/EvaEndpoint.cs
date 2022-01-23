using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.IO;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace ExecutableVanillaApplication;

public sealed class EvaEndpoint
{
	private readonly EvaPipe _pipe;

	public EvaEndpoint()
	{
		var pipeName = $@"\\.\pipe\magi-workspace-manager-{Process.GetCurrentProcess().Id}";

		_pipe = new EvaPipe($@"\\.\pipe\magi-workspace-manager-{Process.GetCurrentProcess().Id}");
	}

	public ManagerMessage Read()
	{

	}

	public static bool IsWorkspaceManagerRunning()
	{
		return File.Exists($@"\\.\pipe\magi-workspace-manager");
	}

	public void UpdateTaskStatus(IEnumerable<string> taskIds)
	{
	}
}
