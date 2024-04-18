import subprocess


def run_command(command):
    print(f"Executing command: {command}")
    process = subprocess.Popen(command, shell=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    stdout, stderr = process.communicate()
    if process.returncode != 0:
        if command.startswith("helm install") and "cannot re-use a name that is still in use" in stderr.decode("utf-8"):
            print(f"{command} / {stderr.decode("utf-8")}")
            return stdout.decode("utf-8")
        if command.startswith("kubectl create secret") and "already exists" in stderr.decode("utf-8"):
            print(f"{command} / {stderr.decode("utf-8")}")
            return stdout.decode("utf-8")

        print(f"Error executing command: {command}")
        print(f"Error message: {stderr.decode('utf-8')}")
        exit(1)
    print(f"Command executed successfully: {command}")
    return stdout.decode('utf-8')


def main(ci=False, cd=False):
    print("Starting script execution...")

    print("Applying cert-manager...")
    run_command("kubectl apply -f https://github.com/cert-manager/cert-manager/releases/download/v1.14.4/cert-manager.yaml")

    print("Waiting for cert-manager deployments...")
    for deployment in ["cert-manager", "cert-manager-cainjector", "cert-manager-webhook"]:
        run_command(f"kubectl wait --for=condition=available --timeout=600s deployment/{deployment} -n cert-manager")

    print("Applying CockroachDB issuer...")
    run_command("kubectl apply -f cockroach/cockroach_issuer.yaml")

    print("Adding Helm repositories and updating...")
    run_command("helm repo add cockroachdb https://charts.cockroachdb.com/")
    run_command("helm repo add grafana https://grafana.github.io/helm-charts")
    run_command("helm repo update")

    print("Installing CockroachDB...")
    run_command("helm install cockroach --values cockroach/cockroach_values.yaml cockroachdb/cockroachdb")

    print("Labeling CockroachDB service for Prometheus...")
    run_command("kubectl label svc cockroach-cockroachdb prometheus=cockroachdb")

    print("Applying Prometheus Operator bundle and CockroachDB-specific Prometheus configurations...")
    run_command("kubectl apply -f https://raw.githubusercontent.com/prometheus-operator/prometheus-operator/v0.73.1/bundle.yaml --server-side")
    run_command("kubectl apply -f cockroach/prometheus.yaml")

    print("Creating secret for Alertmanager configuration...")
    run_command("kubectl create secret generic alertmanager-cockroachdb --from-file=alertmanager.yaml=cockroach/alertmanager-config.yaml")
    run_command("kubectl label secret alertmanager-cockroachdb app=cockroachdb")

    print("Applying Alertmanager configurations...")
    run_command("kubectl apply -f https://raw.githubusercontent.com/cockroachdb/cockroach/master/cloud/kubernetes/prometheus/alertmanager.yaml")
    run_command("kubectl apply -f https://raw.githubusercontent.com/cockroachdb/cockroach/master/cloud/kubernetes/prometheus/alert-rules.yaml")

    print("Installing Grafana...")
    run_command("helm install grafana grafana/grafana")

    print("Applying CockroachDB secure configuration...")
    run_command("kubectl apply -f cockroach/cockroach-secure.yaml")

    print("Executing SQL commands to create a user and a table...")
    run_command("kubectl exec -it cockroachdb-client-secure -- cockroach sql --certs-dir=./cockroach-certs --host=cockroach-cockroachdb-public -e \"CREATE USER roach WITH LOGIN PASSWORD 'password'; CREATE TABLE IF NOT EXISTS links(short_link TEXT, full_link TEXT, last_accessed TIMESTAMP, PRIMARY KEY(short_link)); GRANT SELECT, INSERT, UPDATE, DELETE ON TABLE links TO roach;\"")
    print("Installing Redis...")
    run_command("helm install redis oci://registry-1.docker.io/bitnamicharts/redis")

    if ci:
        print("Installing Jenkins...")
        run_command("helm repo add jenkins https://charts.jenkins.io")
        run_command("helm repo update jenkins")
        run_command("helm install jenkins jenkins/jenkins")
        run_command("kubectl apply -f https://raw.githubusercontent.com/jenkins-infra/jenkins.io/master/content/doc/tutorials/kubernetes/installing-jenkins-on-kubernetes/jenkins-volume.yaml")
        print("Installing sonarqube")
        run_command("helm repo add sonarqube https://SonarSource.github.io/helm-chart-sonarqube")
        run_command("helm repo update sonarqube")
        run_command("helm install sonarqube sonarqube/sonarqube --set sonar.admin.password=password")

    print("Applying the application YAML file...")
    run_command("kubectl apply -f app.yaml")

    print("Script execution completed successfully.")

if __name__ == "__main__":
    main(ci = False, cd=False)
